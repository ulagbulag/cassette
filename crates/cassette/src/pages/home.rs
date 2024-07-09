use yew::prelude::*;

#[function_component(Home)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="/assets/logo.webp" alt="Cassette logo" />
            <h1>{ "Cassette" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
