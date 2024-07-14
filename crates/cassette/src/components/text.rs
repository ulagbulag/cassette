use cassette_core::{
    cassette::CassetteState,
    task::{TaskResult, TaskSpec, TaskState},
};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub fn render(_state: &UseStateHandle<CassetteState>, spec: &TaskSpec) -> TaskResult {
    let msg = spec.get_string("/msg")?;

    Ok(TaskState::Continue {
        body: html! { <Component { msg } /> },
    })
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct Props {
    msg: String,
}

#[function_component(Component)]
fn component(props: &Props) -> Html {
    let Props { msg } = props;

    html! {
        <Content>
            <p>{ msg }</p>
        </Content>
    }
}
