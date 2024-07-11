use inflector::Inflector;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let title = env!("CARGO_PKG_NAME").to_title_case();
    let subtitle = env!("CARGO_PKG_DESCRIPTION");

    html! {
        <super::PageBody {title} {subtitle} >
            <Content>
            </Content>
        </super::PageBody>
    }
}
