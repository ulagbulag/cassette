use cassette_core::{
    cassette::CassetteState,
    task::{TaskResult, TaskSpec, TaskState},
};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub fn render(_state: &UseStateHandle<CassetteState>, spec: &TaskSpec) -> TaskResult {
    let msg = spec.get_string("/msg")?;

    Ok(TaskState::Continue {
        body: html! { <ComponentBody { msg } /> },
    })
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct BodyProps {
    msg: String,
}

#[function_component(ComponentBody)]
fn component_body(props: &BodyProps) -> Html {
    let BodyProps { msg } = props;

    html! {
        <Content>
            <p>{ msg }</p>
        </Content>
    }
}
