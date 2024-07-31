use std::{borrow::Cow, fmt, future::Future, marker::PhantomData, mem, ops, rc::Rc};

#[cfg(feature = "stream")]
use anyhow::Result;
pub use gloo_net::http::Method;
use gloo_net::http::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};
#[cfg(feature = "stream")]
pub use wasm_streams::readable::IntoStream;
#[cfg(feature = "stream")]
use wasm_streams::readable::ReadableStream;
use yew::platform::spawn_local;

use crate::cassette::GenericCassetteTaskHandle;

pub type FetchRequestWithoutBody<Uri> = FetchRequest<Uri, ()>;

pub struct FetchRequest<Uri, Req> {
    pub method: Method,
    pub name: Cow<'static, str>,
    pub uri: Uri,
    pub body: Option<Body<Req>>,
}

pub enum Body<T> {
    Json(T),
}

impl<Uri, Req> FetchRequest<Uri, Req> {
    pub fn try_fetch<State, Res>(self, base_url: &str, state: State)
    where
        State: 'static + GenericCassetteTaskHandle<FetchState<Res>>,
        for<'a> <State as GenericCassetteTaskHandle<FetchState<Res>>>::Ref<'a>:
            ops::Deref<Target = FetchState<Res>>,
        Req: 'static + Serialize,
        Res: 'static + DeserializeOwned,
        Uri: fmt::Display,
    {
        let handler = |result| match result {
            crate::result::HttpResult::Ok(data) => FetchState::Completed(data),
            crate::result::HttpResult::Err(error) => FetchState::Error(error),
        };
        self.try_fetch_with(base_url, state, handler, false)
    }

    pub fn try_fetch_force<State, Res>(self, base_url: &str, state: State)
    where
        State: 'static + GenericCassetteTaskHandle<FetchState<Res>>,
        for<'a> <State as GenericCassetteTaskHandle<FetchState<Res>>>::Ref<'a>:
            ops::Deref<Target = FetchState<Res>>,
        Req: 'static + Serialize,
        Res: 'static + DeserializeOwned,
        Uri: fmt::Display,
    {
        let handler = |result| match result {
            crate::result::HttpResult::Ok(data) => FetchState::Completed(data),
            crate::result::HttpResult::Err(error) => FetchState::Error(error),
        };
        self.try_fetch_with(base_url, state, handler, true)
    }

    pub fn try_fetch_unchecked<State, Res>(self, base_url: &str, state: State)
    where
        State: 'static + GenericCassetteTaskHandle<FetchState<Res>>,
        for<'a> <State as GenericCassetteTaskHandle<FetchState<Res>>>::Ref<'a>:
            ops::Deref<Target = FetchState<Res>>,
        Req: 'static + Serialize,
        Res: 'static + DeserializeOwned,
        Uri: fmt::Display,
    {
        let handler = FetchState::Completed;
        self.try_fetch_with(base_url, state, handler, false)
    }

    pub fn try_fetch_unchecked_force<State, Res>(self, base_url: &str, state: State)
    where
        State: 'static + GenericCassetteTaskHandle<FetchState<Res>>,
        for<'a> <State as GenericCassetteTaskHandle<FetchState<Res>>>::Ref<'a>:
            ops::Deref<Target = FetchState<Res>>,
        Req: 'static + Serialize,
        Res: 'static + DeserializeOwned,
        Uri: fmt::Display,
    {
        let handler = FetchState::Completed;
        self.try_fetch_with(base_url, state, handler, true)
    }

    fn try_fetch_with<State, Res, ResRaw, F>(
        self,
        base_url: &str,
        state: State,
        handler: F,
        force: bool,
    ) where
        State: 'static + GenericCassetteTaskHandle<FetchState<Res>>,
        for<'a> <State as GenericCassetteTaskHandle<FetchState<Res>>>::Ref<'a>:
            ops::Deref<Target = FetchState<Res>>,
        Req: 'static + Serialize,
        Res: 'static + DeserializeOwned,
        ResRaw: 'static + DeserializeOwned,
        Uri: fmt::Display,
        F: 'static + FnOnce(ResRaw) -> FetchState<Res>,
    {
        if force || matches!(*state.get(), FetchState::Pending) {
            state.set(FetchState::Fetching);

            let Self {
                method,
                name,
                uri,
                body,
            } = self;
            let url = format!("{base_url}{uri}");

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
                        Ok(response) => match response.text().await {
                            Ok(text) => match ::serde_json::from_str(&text) {
                                Ok(data) => handler(data),
                                Err(_) => {
                                    if text.is_empty() {
                                        FetchState::Error("No Response".into())
                                    } else {
                                        FetchState::Error(text)
                                    }
                                }
                            },
                            Err(error) => {
                                FetchState::Error(format!("Failed to read the {name}: {error}"))
                            }
                        },
                        Err(error) => {
                            FetchState::Error(format!("Failed to fetch the {name}: {error}"))
                        }
                    },
                    Err(state) => state,
                };
                if force || matches!(*state.get(), FetchState::Pending | FetchState::Fetching) {
                    state.set(value);
                }
            })
        }
    }
}

impl<Uri, Req> FetchRequest<Uri, Req> {
    pub fn try_stream_with<'reader, State, Res, F, Fut>(
        self,
        base_url: &str,
        state: State,
        mut handler: F,
    ) where
        State: 'static + GenericCassetteTaskHandle<FetchState<Res>>,
        for<'a> <State as GenericCassetteTaskHandle<FetchState<Res>>>::Ref<'a>:
            ops::Deref<Target = FetchState<Res>>,
        Req: 'static + Serialize,
        Res: 'static + DeserializeOwned,
        Uri: fmt::Display,
        F: 'static + FnMut(StreamContext<'reader, State, Req, Res>) -> Fut,
        Fut: Future<Output = Result<StreamState<Req, Res>>>,
    {
        if matches!(*state.get(), FetchState::Pending) {
            state.set(FetchState::Fetching);

            let Self {
                method,
                name,
                uri,
                mut body,
            } = self;
            let url = format!("{base_url}{uri}");

            let mut last_data = None;
            let state = state.clone();
            spawn_local(async move {
                loop {
                    let builder = RequestBuilder::new(&url).method(method.clone());
                    let builder = match body.as_ref() {
                        Some(Body::Json(body)) => builder.json(body).map_err(|error| {
                            FetchState::Error(format!("Failed to encode the body {name}: {error}"))
                        }),
                        None => builder.build().map_err(|error| {
                            FetchState::Error(format!(
                                "Failed to build the request {name}: {error}"
                            ))
                        }),
                    };

                    let value = match builder {
                        Ok(builder) => match builder.send().await {
                            Ok(response) => match response
                                .body()
                                .map(ReadableStream::from_raw)
                                .map(ReadableStream::into_stream)
                            {
                                Some(stream) => {
                                    let ctx = StreamContext {
                                        body: body.take(),
                                        data: last_data.take(),
                                        setter: FetchStateSetter::new(state.clone()),
                                        stream,
                                    };
                                    match handler(ctx).await {
                                        Ok(StreamState::Complete(data)) => {
                                            FetchState::Completed(Rc::new(data))
                                        }
                                        Ok(StreamState::Continue(new_body, data)) => {
                                            body.replace(new_body);
                                            last_data.replace(data);
                                            continue;
                                        }
                                        Err(error) => FetchState::Error(format!(
                                            "Failed to parse the {name}: {error}"
                                        )),
                                    }
                                }
                                None => FetchState::Error(format!("Empty body: {name}")),
                            },
                            Err(error) => {
                                FetchState::Error(format!("Failed to fetch the {name}: {error}"))
                            }
                        },
                        Err(state) => state,
                    };
                    if matches!(*state.get(), FetchState::Pending | FetchState::Fetching) {
                        state.set(value);
                    }
                    break;
                }
            })
        }
    }
}

pub struct StreamContext<'reader, State, Req, Res> {
    pub body: Option<Body<Req>>,
    pub data: Option<Res>,
    pub setter: FetchStateSetter<State, Res>,
    pub stream: IntoStream<'reader>,
}

pub enum StreamState<Req, Res> {
    Complete(Res),
    Continue(Body<Req>, Res),
}

pub struct FetchStateSetter<T, Item> {
    _item: PhantomData<FetchState<Item>>,
    inner: T,
}

impl<T, Item> FetchStateSetter<T, Item>
where
    T: GenericCassetteTaskHandle<FetchState<Item>>,
{
    const fn new(inner: T) -> Self {
        Self {
            _item: PhantomData,
            inner,
        }
    }

    pub fn set(&mut self, value: Item)
    where
        T: 'static,
        Item: 'static,
    {
        self.inner.set(FetchState::Collecting(Rc::new(value)))
    }
}

#[derive(Clone, Debug, Default)]
pub enum FetchState<T> {
    #[default]
    Pending,
    Fetching,
    Collecting(Rc<T>),
    Completed(Rc<T>),
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
            Self::Collecting(data) => data.fmt(f),
            Self::Completed(data) => data.fmt(f),
            Self::Error(error) => error.fmt(f),
        }
    }
}

impl<T> PartialEq for FetchState<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // compare the current collection status
            (Self::Collecting(l0), Self::Collecting(r0)) => Rc::ptr_eq(l0, r0),
            // just compare the discriminant
            _ => mem::discriminant(self) == mem::discriminant(other),
        }
    }
}
