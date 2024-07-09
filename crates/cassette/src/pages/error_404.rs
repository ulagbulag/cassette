use yew::prelude::*;

#[function_component(Error404)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "404" }</h1>
            <span class="subtitle">{ "Not Found :/" }</span>
        </main>
    }
}
