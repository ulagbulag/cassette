use anyhow::{anyhow, bail, Result};
use cassette_core::{
    cassette::{CassetteContext, CassetteTaskHandle},
    net::fetch::{
        Body, FetchRequest, FetchState, FetchStateHandle, FetchStateSetter, Method, StreamContext,
        StreamState,
    },
};
use futures::TryStreamExt;
use itertools::Itertools;
use js_sys::Uint8Array;

use crate::schema::{Message, MessageChoice, MessageFinishReason, Request, Response, Role};

pub fn use_fetch(
    ctx: &mut CassetteContext,
    base_url: &str,
    request: Request,
) -> CassetteTaskHandle<FetchState<String>> {
    let handler_name = "chat completions".into();
    let state = ctx.use_state(handler_name, || FetchState::Pending);
    {
        let state = state.clone();
        let base_url = base_url.to_string();
        let stream = request.options.stream;
        let request = FetchRequest {
            method: Method::POST,
            name: "chat completions",
            url: "/chat/completions",
            body: Some(Body::Json(request.clone())),
        };

        match stream {
            Some(true) => request.try_stream_with(&base_url, state, try_stream),
            Some(false) | None => request.try_fetch_unchecked(&base_url, state),
        }
    }
    state
}

const PATTERN: &[u8] = "\n\ndata: ".as_bytes();

async fn try_stream<State>(
    ctx: StreamContext<'_, State, Request, String>,
) -> Result<StreamState<Request, String>>
where
    State: 'static + FetchStateHandle<String>,
{
    let StreamContext {
        body,
        data,
        setter,
        stream,
    } = ctx;
    let data_last = data.as_ref().map(|data| data.len()).unwrap_or_default();

    let mut stream = stream
        .map_ok(|chunk| Uint8Array::new(&chunk).to_vec())
        .map_err(|error| match error.as_string() {
            Some(error) => anyhow!("{error}"),
            None => anyhow!("{error:?}"),
        });

    let mut heystack = "\n\n".to_string().into_bytes();
    let mut output = TokenStream {
        data: data.unwrap_or_default(),
        data_last,
        body,
        finish_reason: None,
        setter,
    };
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

struct TokenStream<State> {
    data: String,
    data_last: usize,
    body: Option<Body<Request>>,
    finish_reason: Option<MessageFinishReason>,
    setter: FetchStateSetter<State, String>,
}

impl<State> TokenStream<State>
where
    State: 'static + FetchStateHandle<String>,
{
    fn feed(&mut self, data: &[u8]) -> Result<()> {
        self.feed_with(data, true)
    }

    fn feed_with(&mut self, data: &[u8], update: bool) -> Result<()> {
        if !data.starts_with(PATTERN) {
            bail!("unexpected opcode");
        }
        let data = &data[PATTERN.len()..];

        // handle sub-opcodes
        if data.starts_with("[DONE]".as_bytes()) {
            return Ok(());
        }

        let Response { mut choices } = ::serde_json::from_slice(data)?;
        if let Some(MessageChoice {
            index: _,
            message,
            finish_reason,
        }) = choices.pop_front()
        {
            self.data.push_str(&message.content);
            self.finish_reason = finish_reason;
            if update {
                self.setter.set(self.data.clone());
            }
        }
        Ok(())
    }

    fn finish(mut self, data: &[u8]) -> Result<StreamState<Request, String>> {
        self.feed_with(data, false)?;

        let Self {
            data,
            data_last,
            body,
            finish_reason,
            mut setter,
        } = self;
        match finish_reason {
            Some(MessageFinishReason::EosToken) | None => Ok(StreamState::Complete(data)),
            Some(MessageFinishReason::Length) => {
                setter.set(data.clone());

                let body = match body {
                    Some(Body::Json(Request {
                        model,
                        options,
                        mut messages,
                    })) => Body::Json(Request {
                        model,
                        options,
                        messages: {
                            messages.push(Message {
                                role: Role::Assistant,
                                content: data[data_last..].to_string() + " ",
                            });
                            messages.push(Message {
                                role: Role::User,
                                content: "continue it briefly".into(), // [CONTINUE]
                            });
                            messages
                        },
                    }),
                    _ => bail!("unexpected body type"),
                };

                Ok(StreamState::Continue(body, data))
            }
        }
    }
}
