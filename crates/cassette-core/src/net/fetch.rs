use std::fmt;

use gloo_net::http::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use yew::{platform::spawn_local, prelude::*};

pub struct FetchRequest<Url> {
    pub method: Method,
    pub name: &'static str,
    pub url: Url,
}

impl<Url> FetchRequest<Url> {
    pub fn try_fetch<Res, UrlOut>(
        self,
        base_url: &str,
        state: UseStateHandle<self::sealed::FetchState<Res>>,
    ) where
        Res: 'static + DeserializeOwned,
        Url: FnOnce() -> UrlOut,
        UrlOut: fmt::Display,
    {
        if matches!(&*state, self::sealed::FetchState::Pending) {
            state.set(self::sealed::FetchState::Fetching);

            let Self { method, name, url } = self;
            let url = format!("{base_url}/{url}", url = url());

            let state = state.clone();
            spawn_local(async move {
                let request = RequestBuilder::new(&url).method(method);
                let value = match request.send().await {
                    Ok(response) => match response.json().await {
                        Ok(data) => self::sealed::FetchState::Completed(data),
                        Err(error) => self::sealed::FetchState::Error(format!(
                            "Failed to parse the {name}: {error}"
                        )),
                    },
                    Err(error) => self::sealed::FetchState::Error(format!(
                        "Failed to fetch the {name}: {error}"
                    )),
                };
                if matches!(&*state, self::sealed::FetchState::Fetching) {
                    state.set(value);
                }
            })
        }
    }
}

pub(super) mod sealed {
    use std::fmt;

    #[derive(Debug, Default)]
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
                FetchState::Pending => "pending".fmt(f),
                FetchState::Fetching => "loading".fmt(f),
                FetchState::Completed(data) => data.fmt(f),
                FetchState::Error(error) => error.fmt(f),
            }
        }
    }
}
