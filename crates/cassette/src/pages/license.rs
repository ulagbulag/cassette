use yew::prelude::*;

#[function_component(License)]
pub fn license() -> Html {
    html! {
        <main class="license">
            <div class="license">{ include_str!("../../../../LICENSE") }</div>
        </main>
    }
}
