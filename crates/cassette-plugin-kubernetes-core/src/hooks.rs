use std::future::Future;

use anyhow::Result;
use cassette_core::net::fetch::FetchState;
use kube_core::{params::ListParams, ObjectList};
use serde::de::DeserializeOwned;
use yew::{platform::spawn_local, prelude::*};

use crate::api::Api;

#[hook]
pub fn use_kubernetes_list<K>(
    api: Api<K>,
    lp: ListParams,
) -> UseStateHandle<FetchState<ObjectList<K>>>
where
    K: 'static + Clone + DeserializeOwned,
{
    let state = use_state(|| FetchState::Pending);
    {
        let state = state.clone();
        let f = move || api.list(lp);
        use_effect(move || try_fetch(state, f))
    }
    state
}

fn try_fetch<F, Fut, Res>(state: UseStateHandle<FetchState<Res>>, f: F)
where
    F: 'static + FnOnce() -> Fut,
    Fut: 'static + Future<Output = Result<Res>>,
    Res: 'static,
{
    if matches!(&*state, FetchState::Pending) {
        state.set(FetchState::Fetching);

        let state = state.clone();
        spawn_local(async move {
            let value = match f().await {
                Ok(data) => FetchState::Completed(data),
                Err(error) => FetchState::Error(error.to_string()),
            };
            if matches!(&*state, FetchState::Pending | FetchState::Fetching) {
                state.set(value);
            }
        })
    }
}
