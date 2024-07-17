use std::{fmt, future::Future};

pub use gloo_net::http::Method;
use gloo_net::http::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};
#[cfg(feature = "stream")]
pub use wasm_streams::readable::IntoStream;
#[cfg(feature = "stream")]
use wasm_streams::readable::ReadableStream;
use yew::{platform::spawn_local, prelude::*};

pub type FetchRequestWithoutBody<Url> = FetchRequest<Url, ()>;

pub struct FetchRequest<Url, Req> {
    pub method: Method,
    pub name: &'static str,
    pub url: Url,
    pub body: Option<Body<Req>>,
}

pub enum Body<T> {
    Json(T),
}

impl<Url, Req> FetchRequest<Url, Req> {
    pub fn try_fetch<Res>(self, base_url: &str, state: UseStateHandle<FetchState<Res>>)
    where
        Req: 'static + Serialize,
        Res: 'static + DeserializeOwned,
        Url: fmt::Display,
    {
        let handler = |result| match result {
            crate::result::Result::Ok(data) => FetchState::Completed(data),
            crate::result::Result::Err(error) => FetchState::Error(error),
        };
        self.try_fetch_with(base_url, state, handler)
    }

    pub fn try_fetch_unchecked<Res>(self, base_url: &str, state: UseStateHandle<FetchState<Res>>)
    where
        Req: 'static + Serialize,
        Res: 'static + DeserializeOwned,
        Url: fmt::Display,
    {
        let handler = FetchState::Completed;
        self.try_fetch_with(base_url, state, handler)
    }

    fn try_fetch_with<Res, ResRaw, F>(
        self,
        base_url: &str,
        state: UseStateHandle<FetchState<Res>>,
        handler: F,
    ) where
        Req: 'static + Serialize,
        Res: 'static + DeserializeOwned,
        ResRaw: 'static + DeserializeOwned,
        Url: fmt::Display,
        F: 'static + FnOnce(ResRaw) -> FetchState<Res>,
    {
        if matches!(&*state, FetchState::Pending) {
            state.set(FetchState::Fetching);

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
                        FetchState::Error(format!("Failed to encode the body {name}: {error}"))
                    }),
                    None => builder.build().map_err(|error| {
                        FetchState::Error(format!("Failed to build the request {name}: {error}"))
                    }),
                };

                let value = match builder {
                    Ok(builder) => match builder.send().await {
                        Ok(response) => match response.json().await {
                            Ok(data) => handler(data),
                            Err(error) => {
                                FetchState::Error(format!("Failed to parse the {name}: {error}"))
                            }
                        },
                        Err(error) => {
                            FetchState::Error(format!("Failed to fetch the {name}: {error}"))
                        }
                    },
                    Err(state) => state,
                };
                if matches!(&*state, FetchState::Pending | FetchState::Fetching) {
                    state.set(value);
                }
            })
        }
    }
}

impl<Url, Req> FetchRequest<Url, Req> {
    pub fn try_stream_with<'reader, Res, F, Fut>(
        self,
        base_url: &str,
        state: UseStateHandle<FetchState<Res>>,
        handler: F,
    ) where
        Req: 'static + Serialize,
        Res: 'static + DeserializeOwned,
        Url: fmt::Display,
        F: 'static + FnOnce(UseStateSetter<FetchState<Res>>, IntoStream<'reader>) -> Fut,
        Fut: Future<Output = ::anyhow::Result<()>>,
    {
        if matches!(&*state, FetchState::Pending) {
            state.set(FetchState::Fetching);

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
                        FetchState::Error(format!("Failed to encode the body {name}: {error}"))
                    }),
                    None => builder.build().map_err(|error| {
                        FetchState::Error(format!("Failed to build the request {name}: {error}"))
                    }),
                };

                let value = match builder {
                    Ok(builder) => match builder.send().await {
                        Ok(response) => match response
                            .body()
                            .map(ReadableStream::from_raw)
                            .map(ReadableStream::into_stream)
                        {
                            Some(body) => match handler(state.setter(), body).await {
                                Ok(()) => return,
                                Err(error) => FetchState::Error(format!(
                                    "Failed to parse the {name}: {error}"
                                )),
                            },
                            None => FetchState::Error(format!("Empty body: {name}")),
                        },
                        Err(error) => {
                            FetchState::Error(format!("Failed to fetch the {name}: {error}"))
                        }
                    },
                    Err(state) => state,
                };
                if matches!(&*state, FetchState::Pending | FetchState::Fetching) {
                    state.set(value);
                }
            })
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum FetchState<T> {
    #[default]
    Pending,
    Fetching,
    Completed(T),
    Error(String),
}

impl<T> fmt::Display for FetchState<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => "pending".fmt(f),
            Self::Fetching => "loading".fmt(f),
            Self::Completed(data) => data.fmt(f),
            Self::Error(error) => error.fmt(f),
        }
    }
}
