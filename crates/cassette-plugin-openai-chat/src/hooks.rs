use anyhow::{anyhow, bail, Result};
use cassette_core::net::fetch::{
    Body, FetchRequest, FetchState, FetchStateSetter, IntoStream, Method,
};
use futures::TryStreamExt;
use itertools::Itertools;
use js_sys::Uint8Array;
use yew::prelude::*;

use crate::schema::{Request, Response};

#[hook]
pub fn use_fetch(base_url: &str, request: Request) -> UseStateHandle<FetchState<Response>> {
    let state = use_state(|| FetchState::Pending);
    {
        let state = state.clone();
        let base_url = base_url.to_string();
        let stream = request.options.stream;
        let request = FetchRequest {
            method: Method::POST,
            name: "chat completions",
            url: "/chat/completions",
            body: Some(Body::Json(request)),
        };

        let f: Box<dyn FnOnce()> = if stream {
            Box::new(move || request.try_stream_with(&base_url, state, try_stream))
        } else {
            Box::new(move || request.try_fetch_unchecked(&base_url, state))
        };
        use_effect(f)
    }
    state
}

async fn try_stream(
    setter: FetchStateSetter<Response>,
    stream: IntoStream<'_>,
) -> Result<Response> {
    let mut stream = stream
        .map_ok(|chunk| Uint8Array::new(&chunk).to_vec())
        .map_err(|error| match error.as_string() {
            Some(error) => anyhow!("{error}"),
            None => anyhow!("{error:?}"),
        });

    const PATTERN: &[u8] = "\n\ndata: ".as_bytes();

    struct TokenStream {
        output: Response,
        setter: FetchStateSetter<Response>,
    }

    impl TokenStream {
        fn new(setter: FetchStateSetter<Response>) -> Self {
            Self {
                output: Response::default(),
                setter,
            }
        }

        fn feed(&mut self, data: &[u8]) -> Result<()> {
            self.feed_with(data, true)
        }

        fn feed_with(&mut self, data: &[u8], update: bool) -> Result<()> {
            if !data.starts_with(PATTERN) {
                bail!("unexpected opcode");
            }
            let data = &data[PATTERN.len()..];

            let Response { mut choices } = ::serde_json::from_slice(data)?;
            if let Some(choice) = choices.pop_front() {
                self.output.choices.push_back(choice);
                if update {
                    self.setter.set(self.output.clone());
                }
            }
            Ok(())
        }

        fn finish(mut self, data: &[u8]) -> Result<Response> {
            self.feed_with(data, false)?;
            Ok(self.output)
        }
    }

    let mut heystack = "\n\n".to_string().into_bytes();
    let mut output = TokenStream::new(setter);
    while let Some(chunk) = stream.try_next().await? {
        heystack.extend(chunk);

        let token_indices: Vec<_> = heystack
            .windows(PATTERN.len())
            .positions(|window| window == PATTERN)
            .collect();

        // [PATTERN, first, PATTERN, ...] => take "first"
        if token_indices.len() >= 2 {
            let mut offset = 0;
            for (start, end) in token_indices.into_iter().tuple_windows() {
                output.feed(&heystack[start..end])?;
                offset = end;
            }
            heystack.drain(..offset);
        }
    }
    output.finish(&heystack)
}
