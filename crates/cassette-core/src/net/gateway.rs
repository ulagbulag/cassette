use std::{collections::HashMap, fmt};

pub use gloo_net::http::Method;
use serde::de::DeserializeOwned;
use yew::prelude::*;
use yew_router::prelude::*;

use super::fetch::{sealed::FetchState, FetchRequest};

#[hook]
pub fn use_fetch<Res, Url, UrlOut>(
    request: impl FnOnce() -> FetchRequest<Url>,
) -> UseStateHandle<FetchState<Res>>
where
    Res: 'static + DeserializeOwned,
    Url: 'static + FnOnce() -> UrlOut,
    UrlOut: 'static + fmt::Display,
{
    let state = use_state(|| FetchState::<Res>::Pending);
    {
        let request = FetchRequest {
            method: Method::GET,
            name: "gateway health",
            url: || "/_health",
        };

        let gateway_url = use_gateway();
        let state = state.clone();
        use_effect(move || request.try_fetch(&gateway_url, state))
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
        .unwrap_or_else(|| "/v1/casette".into())
}

#[hook]
pub fn use_gateway_status() -> String {
    let state = use_fetch::<String, _, _>(|| FetchRequest {
        method: Method::GET,
        name: "gateway health",
        url: || "/_health",
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
