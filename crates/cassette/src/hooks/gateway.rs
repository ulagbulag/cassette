use cassette_core::{
    cassette::CassetteRef,
    net::{
        fetch::{FetchRequest, FetchState},
        gateway::{use_fetch, use_namespace, Method},
    },
};
use yew::prelude::*;

#[hook]
pub fn use_cassette_list() -> UseStateHandle<FetchState<Vec<CassetteRef>>> {
    let namespace = use_namespace();
    use_fetch(move || FetchRequest {
        method: Method::GET,
        name: "cassette list",
        url: format!("/c/{namespace}/"),
    })
}
