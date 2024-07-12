use cassette_core::{
    cassette::{Cassette, CassetteRef},
    net::{
        fetch::{FetchRequest, FetchState},
        gateway::{use_fetch, use_namespace, Method},
    },
};
use uuid::Uuid;
use yew::prelude::*;

#[hook]
pub fn use_cassette(id: Uuid) -> UseStateHandle<FetchState<Option<Cassette>>> {
    let namespace = use_namespace();
    use_fetch(move || FetchRequest {
        method: Method::GET,
        name: "get",
        url: format!("/c/{namespace}/{id}"),
    })
}

#[hook]
pub fn use_cassette_list() -> UseStateHandle<FetchState<Vec<CassetteRef>>> {
    let namespace = use_namespace();
    use_fetch(move || FetchRequest {
        method: Method::GET,
        name: "list",
        url: format!("/c/{namespace}/"),
    })
}
