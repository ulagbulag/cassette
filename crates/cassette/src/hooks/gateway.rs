#[cfg(not(feature = "examples"))]
use cassette_core::net::{
    fetch::{FetchRequestWithoutBody, Method},
    gateway::use_fetch,
};
use cassette_core::{
    cassette::{Cassette, CassetteRef},
    net::{fetch::FetchState, gateway::get_namespace},
};
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CassetteState {
    id: Uuid,
    pub(crate) data: FetchState<Option<Cassette>>,
}

#[hook]
pub fn use_cassette(id: Uuid) -> UseStateHandle<CassetteState> {
    let namespace = get_namespace();
    let state = use_state_eq(|| CassetteState {
        id,
        data: FetchState::Pending,
    });

    #[cfg(feature = "examples")]
    {
        state.set(CassetteState {
            id,
            data: FetchState::Completed(::cassette_loader_file::db().get(&namespace, id).into()),
        })
    }

    #[cfg(not(feature = "examples"))]
    {
        let gateway_url = get_gateway();
        let state = state.clone();
        let request = FetchRequestWithoutBody {
            method: Method::GET,
            name: "get",
            url: format!("/c/{namespace}/{id}"),
            body: None,
        };
        use_effect(move || request().try_fetch(&gateway_url, state))
    }
    state
}

#[hook]
pub fn use_cassette_list() -> UseStateHandle<FetchState<Vec<CassetteRef>>> {
    let namespace = get_namespace();

    #[cfg(feature = "examples")]
    {
        use_state(|| FetchState::Completed(::cassette_loader_file::db().list(&namespace).into()))
    }

    #[cfg(not(feature = "examples"))]
    {
        use_fetch(move || FetchRequestWithoutBody {
            method: Method::GET,
            name: "list",
            url: format!("/c/{namespace}/"),
            body: None,
        })
    }
}
