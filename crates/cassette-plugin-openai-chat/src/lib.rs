mod hooks;
mod schema;

use cassette_core::{
    cassette::CassetteState,
    net::fetch::FetchState,
    task::{TaskResult, TaskSpec, TaskState},
};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_markdown::Markdown;

use crate::schema::Request;

pub fn render(_state: &UseStateHandle<CassetteState>, spec: &TaskSpec) -> TaskResult {
    let base_url = spec.get_string("/baseUrl")?;
    let request: Request = spec.get_model("/")?;

    Ok(TaskState::Continue {
        body: html! { <Component { base_url } { request } /> },
    })
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct Props {
    base_url: String,
    request: Request,
}

#[function_component(Component)]
fn component(props: &Props) -> Html {
    let Props { base_url, request } = props;

    let value = self::hooks::use_fetch(base_url, request);
    match &*value {
        FetchState::Pending | FetchState::Fetching => html! {
            <Content>
                <p>{ "Loading..." }</p>
            </Content>
        },
        FetchState::Collecting(content) => html! {
            <ComponentBody completed=false content={ content.clone() } />
        },
        FetchState::Completed(content) => html! {
            <ComponentBody completed=true content={ content.clone() } />
        },
        FetchState::Error(error) => html! {
            <Alert inline=true title="Error" r#type={AlertType::Danger}>
                { error.clone() }
            </Alert>
        },
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct BodyProps {
    completed: bool,
    content: String,
}

#[function_component(ComponentBody)]
fn component_body(props: &BodyProps) -> Html {
    let BodyProps { completed, content } = props;

    let style = if *completed { "" } else { "color: #FF3333;" };

    html! {
        <Content>
            <div { style }>
                <Markdown src={ content.clone() } />
            </div>
        </Content>
    }
}
