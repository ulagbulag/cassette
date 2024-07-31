use std::fmt;

use serde::{de::DeserializeOwned, Serialize};
use yew::prelude::*;

use super::fetch::{FetchRequest, FetchState};

#[hook]
pub fn use_fetch<Req, Res, Uri>(
    request: impl 'static + FnOnce() -> FetchRequest<Uri, Req>,
) -> UseStateHandle<FetchState<Res>>
where
    Req: 'static + Serialize,
    Res: 'static + DeserializeOwned,
    Uri: 'static + fmt::Display,
{
    let state = use_state(|| FetchState::<Res>::Pending);
    {
        let gateway_url = get_gateway();
        let state = state.clone();
        use_effect(move || request().try_fetch(&gateway_url, state))
    }
    state
}

#[hook]
fn use_fetch_unchecked<Req, Res, Uri>(
    request: impl 'static + FnOnce() -> FetchRequest<Uri, Req>,
) -> UseStateHandle<FetchState<Res>>
where
    Req: 'static + Serialize,
    Res: 'static + DeserializeOwned,
    Uri: 'static + fmt::Display,
{
    let state = use_state(|| FetchState::<Res>::Pending);
    {
        let gateway_url = get_gateway();
        let state = state.clone();
        use_effect(move || request().try_fetch_unchecked(&gateway_url, state))
    }
    state
}

pub fn get_query(key: &str) -> Option<String> {
    // Load current window object
    let window = ::web_sys::window()?;
    // Load current URL
    let href = window.location().href().ok()?;

    // Create an URL object
    let url = ::web_sys::Url::new(&href).ok()?;

    // Take query parameters
    let search_params = url.search_params();

    // Get specific query parameter
    search_params.get(key)
}

pub fn get_gateway() -> String {
    get_query("gateway").unwrap_or_else(|| {
        if cfg!(debug_assertions) && !cfg!(feature = "mock-release") {
            "http://localhost:8080".into()
        } else {
            "/v1/cassette".into()
        }
    })
}

pub fn get_namespace() -> String {
    #[cfg(feature = "examples")]
    {
        super::DEFAULT_NAMESPACE.into()
    }

    #[cfg(not(feature = "examples"))]
    {
        get_query("namespace").unwrap_or_else(|| super::DEFAULT_NAMESPACE.into())
    }
}

pub const fn is_demo() -> bool {
    cfg!(feature = "examples")
}

#[hook]
pub fn use_gateway_status() -> String {
    #[cfg(feature = "examples")]
    {
        "healthy".into()
    }

    #[cfg(not(feature = "examples"))]
    {
        use std::borrow::Cow;

        let state = use_fetch_unchecked::<(), String, _>(move || super::fetch::FetchRequest {
            method: super::fetch::Method::GET,
            name: Cow::Borrowed("gateway health"),
            uri: "/_health",
            body: None,
        });
        state.to_string()
    }
}
