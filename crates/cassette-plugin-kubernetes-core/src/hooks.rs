use std::{future::Future, rc::Rc};

use anyhow::Result;
use cassette_core::{
    cassette::{CassetteContext, CassetteTaskHandle},
    net::fetch::{FetchState, FetchStateHandle},
};
use kube_core::{params::ListParams, ObjectList};
use serde::de::DeserializeOwned;
use yew::platform::spawn_local;

use crate::api::Api;

pub fn use_kubernetes_list<K>(
    ctx: &mut CassetteContext,
    api: Api<K>,
    lp: ListParams,
) -> CassetteTaskHandle<FetchState<ObjectList<K>>>
where
    K: 'static + Clone + DeserializeOwned,
{
    let handler_name = "kubernetes list".into();
    let state = ctx.use_state(handler_name, || FetchState::Pending);
    {
        let state = state.clone();
        let f = move || api.list(lp);
        try_fetch(state, f);
    }
    state
}

fn try_fetch<F, Fut, Res, State>(mut state: State, f: F)
where
    F: 'static + FnOnce() -> Fut,
    Fut: 'static + Future<Output = Result<Res>>,
    Res: 'static,
    State: 'static + FetchStateHandle<Res>,
{
    if matches!(state.get(), FetchState::Pending) {
        state.set(FetchState::Fetching);

        let mut state = state.clone();
        spawn_local(async move {
            let value = match f().await {
                Ok(data) => FetchState::Completed(Rc::new(data)),
                Err(error) => FetchState::Error(error.to_string()),
            };
            if matches!(state.get(), FetchState::Pending | FetchState::Fetching) {
                state.set(value);
            }
        })
    }
}
