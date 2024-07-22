use std::{future::Future, ops, rc::Rc};

use anyhow::Result;
use cassette_core::{
    cassette::{CassetteContext, CassetteTaskHandle, GenericCassetteTaskHandle},
    net::fetch::FetchState,
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
    let handler_name = "kubernetes list";
    let force_init = false;
    let state = ctx.use_state(handler_name, force_init, || FetchState::Pending);
    {
        let state = state.clone();
        let f = move || api.list(lp);
        try_fetch(state, f);
    }
    state
}

fn try_fetch<F, Fut, Res, State>(state: State, f: F)
where
    F: 'static + FnOnce() -> Fut,
    Fut: 'static + Future<Output = Result<Res>>,
    Res: 'static,
    State: 'static + GenericCassetteTaskHandle<FetchState<Res>>,
    for<'a> <State as GenericCassetteTaskHandle<FetchState<Res>>>::Ref<'a>:
        ops::Deref<Target = FetchState<Res>>,
{
    if matches!(*state.get(), FetchState::Pending) {
        state.set(FetchState::Fetching);

        let state = state.clone();
        spawn_local(async move {
            let value = match f().await {
                Ok(data) => FetchState::Completed(Rc::new(data)),
                Err(error) => FetchState::Error(error.to_string()),
            };
            if matches!(*state.get(), FetchState::Pending | FetchState::Fetching) {
                state.set(value);
            }
        })
    }
}
