use yew::prelude::*;

#[function_component(License)]
pub fn license() -> Html {
    let title = "License";

    html! {
        <super::PageBody {title} >
            <div style="
                text-align: left;
                white-space: pre-wrap;
            ">
                { include_str!("../../../../LICENSE") }
            </div>
        </super::PageBody>
    }
}
