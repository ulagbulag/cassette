use cassette_core::{cassette::CassetteRef, net::fetch::FetchState};
use yew::prelude::*;

use crate::hooks::gateway::use_cassette_list;

#[function_component(Home)]
pub fn home() -> Html {
    let title = "Cassette";
    let subtitle = env!("CARGO_PKG_DESCRIPTION");

    let cassette_list = match &*use_cassette_list() {
        FetchState::Pending | FetchState::Fetching => html! { <p>{ "Loading..." }</p> },
        FetchState::Completed(list) => render_cassette_list(list),
        FetchState::Error(error) => html! { <p>{ format!("Error: {error}") }</p> },
    };

    html! {
        <super::PageBody {title} {subtitle} >
            <div class="home">
                <img class="logo" src="/assets/images/icons/logo.webp" alt="Cassette logo" style="
                    width: 5vw;
                    height: 5vw;
                "/>
                { cassette_list }
            </div>
        </super::PageBody>
    }
}

fn render_cassette_list(list: &[CassetteRef]) -> Html {
    html! { <p>{ format!("{list:?}") }</p> }
}
