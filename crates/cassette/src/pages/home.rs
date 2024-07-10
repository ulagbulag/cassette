use cassette_core::net::fetch::FetchState;
use yew::prelude::*;

use crate::hooks::gateway::use_cassette_list;

#[function_component(Home)]
pub fn home() -> Html {
    let cassette_list = match &*use_cassette_list() {
        FetchState::Pending | FetchState::Fetching => html! { <p>{ "Loading..." }</p> },
        FetchState::Completed(list) => html! { <p>{ format!("{list:?}") }</p> },
        FetchState::Error(error) => html! { <p>{ format!("Error: {error}") }</p> },
    };

    html! {
        <main class="home">
            <div class="home">
                <img class="logo" src="/assets/logo.webp" alt="Cassette logo" />
                <h1 class="home">{ "📼 Cassette" }</h1>
                { cassette_list }
            </div>
        </main>
    }
}
