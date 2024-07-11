use std::fmt;

pub use gloo_net::http::Method;
use serde::de::DeserializeOwned;
use yew::prelude::*;

use super::fetch::{FetchRequest, FetchState};

#[hook]
pub fn use_fetch<Res, Url>(
    request: impl 'static + FnOnce() -> FetchRequest<Url>,
) -> UseStateHandle<FetchState<Res>>
where
    Res: 'static + DeserializeOwned,
    Url: 'static + fmt::Display,
{
    let state = use_state(|| FetchState::<Res>::Pending);
    {
        let gateway_url = use_gateway();
        let state = state.clone();
        use_effect(move || request().try_fetch(&gateway_url, state))
    }
    state
}

#[hook]
fn use_fetch_unchecked<Res, Url>(
    request: impl 'static + FnOnce() -> FetchRequest<Url>,
) -> UseStateHandle<FetchState<Res>>
where
    Res: 'static + DeserializeOwned,
    Url: 'static + fmt::Display,
{
    let state = use_state(|| FetchState::<Res>::Pending);
    {
        let gateway_url = use_gateway();
        let state = state.clone();
        use_effect(move || request().try_fetch_unchecked(&gateway_url, state))
    }
    state
}

#[hook]
pub fn use_query(key: &str) -> Option<String> {
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

#[hook]
pub fn use_gateway() -> String {
    use_query("gateway").unwrap_or_else(|| "/v1/cassette".into())
}

#[hook]
pub fn use_gateway_status() -> String {
    let state = use_fetch_unchecked::<String, _>(move || FetchRequest {
        method: Method::GET,
        name: "gateway health",
        url: "/_health",
    });
    state.to_string()
}

#[hook]
pub fn use_namespace() -> String {
    use_query("namespace").unwrap_or_else(|| "default".into())
}
