use std::fmt;

use gloo_net::http::{Method, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use wasm_streams::{readable::IntoAsyncRead, ReadableStream};
use yew::{platform::spawn_local, prelude::*};

pub type StreamRequestWithoutBody<Url> = StreamRequest<Url, ()>;

pub struct StreamRequest<Url, Req> {
    pub method: Method,
    pub name: &'static str,
    pub url: Url,
    pub body: Option<Body<Req>>,
}

pub enum Body<T> {
    Json(T),
}

impl<Url, Req> StreamRequest<Url, Req> {
    pub fn try_fetch_with<Res, F>(
        self,
        base_url: &str,
        state: UseStateHandle<StreamState<Res>>,
        handler: F,
    ) where
        Req: 'static + Serialize,
        Res: 'static + DeserializeOwned,
        Url: fmt::Display,
        F: 'static + FnOnce(IntoAsyncRead) -> StreamState<Res>,
    {
        if matches!(&*state, StreamState::Pending) {
            state.set(StreamState::Streaming);

            let Self {
                method,
                name,
                url: suffix_url,
                body,
            } = self;
            let url = format!("{base_url}{suffix_url}");

            let state = state.clone();
            spawn_local(async move {
                let builder = RequestBuilder::new(&url).method(method);
                let builder = match body {
                    Some(Body::Json(body)) => builder.json(&body).map_err(|error| {
                        StreamState::Error(format!("Failed to encode the body {name}: {error}"))
                    }),
                    None => builder.build().map_err(|error| {
                        StreamState::Error(format!("Failed to build the request {name}: {error}"))
                    }),
                };

                let value = match builder {
                    Ok(builder) => match builder.send().await {
                        Ok(response) => match response
                            .body()
                            .map(ReadableStream::from_raw)
                            .map(ReadableStream::into_async_read)
                        {
                            Some(body) => handler(body),
                            None => StreamState::Error(format!("Empty body: {name}")),
                        },
                        Err(error) => {
                            StreamState::Error(format!("Failed to fetch the {name}: {error}"))
                        }
                    },
                    Err(state) => state,
                };
                if matches!(&*state, StreamState::Pending | StreamState::Streaming) {
                    state.set(value);
                }
            })
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum StreamState<T> {
    #[default]
    Pending,
    Streaming,
    Processing(T),
    Completed(T),
    Error(String),
}

impl<T> fmt::Display for StreamState<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => "pending".fmt(f),
            Self::Streaming => "loading".fmt(f),
            Self::Processing(data) => data.fmt(f),
            Self::Completed(data) => data.fmt(f),
            Self::Error(error) => error.fmt(f),
        }
    }
}
