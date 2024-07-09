use yew::prelude::*;

#[function_component(Component)]
pub fn component() -> Html {
    html! {
        <main>
            <h1>{ "404" }</h1>
            <span class="subtitle">{ "Not Found :/" }</span>
        </main>
    }
}
