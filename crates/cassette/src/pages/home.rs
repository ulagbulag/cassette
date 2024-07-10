use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main class="home">
            <div class="home">
                <img class="logo" src="/assets/logo.webp" alt="Cassette logo" />
                <h1 class="home">{ "Cassette" }</h1>
                // <span class="subtitle">{ health.as_deref().unwrap_or("Pending") }</span>
                <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            </div>
        </main>
    }
}
