#[cfg(not(feature = "examples"))]
use std::ops;

#[cfg(not(feature = "examples"))]
use cassette_core::{
    cassette::GenericCassetteTaskHandle,
    net::{
        fetch::{FetchRequestWithoutBody, Method},
        gateway::use_fetch,
    },
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

    let reset_state = || CassetteState {
        id,
        data: FetchState::Pending,
    };
    let state = use_state_eq(reset_state);
    if state.id != id {
        state.set(reset_state());
    }

    #[cfg(feature = "examples")]
    {
        state.set(CassetteState {
            id,
            data: FetchState::Completed(::cassette_loader_file::db().get(&namespace, id).into()),
        })
    }

    #[cfg(not(feature = "examples"))]
    {
        let gateway_url = cassette_core::net::gateway::get_gateway();
        let state = CassetteStateHandle(state.clone());
        let request = FetchRequestWithoutBody {
            method: Method::GET,
            name: "get",
            url: format!("/c/{namespace}/{id}"),
            body: None,
        };
        use_effect(move || request.try_fetch(&gateway_url, state))
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

#[cfg(not(feature = "examples"))]
#[derive(Clone)]
struct CassetteStateHandle(UseStateHandle<CassetteState>);

#[cfg(not(feature = "examples"))]
impl GenericCassetteTaskHandle<FetchState<Option<Cassette>>> for CassetteStateHandle {
    type Ref<'a> = &'a FetchState<Option<Cassette>>;

    fn get<'a>(
        &'a self,
    ) -> <Self as GenericCassetteTaskHandle<FetchState<Option<Cassette>>>>::Ref<'a>
    where
        <Self as GenericCassetteTaskHandle<FetchState<Option<Cassette>>>>::Ref<'a>:
            ops::Deref<Target = FetchState<Option<Cassette>>>,
    {
        &self.0.data
    }

    fn set(&self, value: FetchState<Option<Cassette>>) {
        self.0.set({
            let mut state = (*self.0).clone();
            state.data = value;
            state
        })
    }
}
