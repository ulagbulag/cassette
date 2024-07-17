use anyhow::{anyhow, bail, Result};
use cassette_core::net::fetch::{Body, FetchRequest, FetchState, IntoStream, Method};
use futures::TryStreamExt;
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
    state: UseStateSetter<FetchState<Response>>,
    stream: IntoStream<'_>,
) -> Result<()> {
    let mut stream = stream
        .map_ok(|chunk| Uint8Array::new(&chunk).to_vec())
        .map_err(|error| match error.as_string() {
            Some(error) => anyhow!("{error}"),
            None => anyhow!("{error:?}"),
        });

    let mut output = Response::default();
    while let Some(chunk) = stream.try_next().await? {
        let (opcode, data) = chunk.split_at(6);
        match ::core::str::from_utf8(opcode)? {
            "data: " => {
                let Response { mut choices } = ::serde_json::from_slice(&data)?;
                if let Some(choice) = choices.pop_front() {
                    output.choices.push_back(choice);
                    state.set(FetchState::Completed(output.clone()));
                }
            }
            opcode => bail!("unexpected opcode: {opcode:?}"),
        }
    }
    Ok(())
}
