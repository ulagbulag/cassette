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
    pub fn try_fetch<Res>(self, base_url: &str, state: UseStateHandle<FetchState<Res>>)
    where
        Res: 'static + DeserializeOwned,
        Url: fmt::Display,
    {
        if matches!(&*state, FetchState::Pending) {
            state.set(FetchState::Fetching);

            let Self {
                method,
                name,
                url: suffix_url,
            } = self;
            let url = format!("{base_url}{suffix_url}");

            let state = state.clone();
            spawn_local(async move {
                let request = RequestBuilder::new(&url).method(method);
                let value = match request.send().await {
                    Ok(response) => match response.json().await {
                        Ok(crate::result::Result::Ok(data)) => FetchState::Completed(data),
                        Ok(crate::result::Result::Err(error)) => FetchState::Error(error),
                        Err(error) => {
                            FetchState::Error(format!("Failed to parse the {name}: {error}"))
                        }
                    },
                    Err(error) => FetchState::Error(format!("Failed to fetch the {name}: {error}")),
                };
                if matches!(&*state, FetchState::Pending | FetchState::Fetching) {
                    state.set(value);
                }
            })
        }
    }

    pub fn try_fetch_unchecked<Res>(self, base_url: &str, state: UseStateHandle<FetchState<Res>>)
    where
        Res: 'static + DeserializeOwned,
        Url: fmt::Display,
    {
        if matches!(&*state, FetchState::Pending) {
            state.set(FetchState::Fetching);

            let Self {
                method,
                name,
                url: suffix_url,
            } = self;
            let url = format!("{base_url}{suffix_url}");

            let state = state.clone();
            spawn_local(async move {
                let request = RequestBuilder::new(&url).method(method);
                let value = match request.send().await {
                    Ok(response) => match response.json().await {
                        Ok(data) => FetchState::Completed(data),
                        Err(error) => {
                            FetchState::Error(format!("Failed to parse the {name}: {error}"))
                        }
                    },
                    Err(error) => FetchState::Error(format!("Failed to fetch the {name}: {error}")),
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
            FetchState::Pending => "pending".fmt(f),
            FetchState::Fetching => "loading".fmt(f),
            FetchState::Completed(data) => data.fmt(f),
            FetchState::Error(error) => error.fmt(f),
        }
    }
}
