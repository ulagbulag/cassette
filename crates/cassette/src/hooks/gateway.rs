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

#[hook]
pub fn use_cassette(id: Uuid) -> UseStateHandle<FetchState<Option<Cassette>>> {
    let namespace = get_namespace();

    #[cfg(feature = "examples")]
    {
        use_state(|| FetchState::Completed(::cassette_loader_file::db().get(&namespace, id)))
    }

    #[cfg(not(feature = "examples"))]
    {
        use_fetch(move || FetchRequestWithoutBody {
            method: Method::GET,
            name: "get",
            url: format!("/c/{namespace}/{id}"),
            body: None,
        })
    }
}

#[hook]
pub fn use_cassette_list() -> UseStateHandle<FetchState<Vec<CassetteRef>>> {
    let namespace = get_namespace();

    #[cfg(feature = "examples")]
    {
        use_state(|| FetchState::Completed(::cassette_loader_file::db().list(&namespace)))
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
