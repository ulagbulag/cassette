use std::{collections::HashMap, fmt};

pub use gloo_net::http::Method;
use serde::de::DeserializeOwned;
use yew::prelude::*;
use yew_router::prelude::*;

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
pub fn use_query() -> HashMap<String, String> {
    let location = use_location();
    location
        .and_then(|location| location.query::<HashMap<String, String>>().ok())
        .unwrap_or_default()
}

#[hook]
pub fn use_gateway() -> String {
    use_query()
        .get("gateway")
        .cloned()
        .unwrap_or_else(|| "/v1/cassette".into())
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
    use_query()
        .get("namespace")
        .cloned()
        .unwrap_or_else(|| "default".into())
}
