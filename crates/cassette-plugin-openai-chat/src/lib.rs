mod hooks;
mod schema;

use cassette_core::{
    cassette::CassetteState,
    net::fetch::FetchState,
    task::{TaskResult, TaskSpec, TaskState},
};
use itertools::Itertools;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_markdown::Markdown;

use crate::schema::{Message, Request, RequestOptions, Response};

pub fn render(_state: &UseStateHandle<CassetteState>, spec: &TaskSpec) -> TaskResult {
    let base_url = spec.get_string("/baseUrl")?;
    let messages: Vec<Message> = spec.get_model("/messages")?;

    Ok(TaskState::Continue {
        body: html! { <Component { base_url } { messages } /> },
    })
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct Props {
    base_url: String,
    messages: Vec<Message>,
}

#[function_component(Component)]
fn component(props: &Props) -> Html {
    let Props { base_url, messages } = props;

    let request = Request {
        model: "tgi".into(),
        options: RequestOptions { stream: true },
        messages: messages.clone(),
    };

    let value = self::hooks::use_fetch(base_url, request);
    match &*value {
        FetchState::Pending | FetchState::Fetching => html! {
            <Content>
                <p>{ "Loading..." }</p>
            </Content>
        },
        FetchState::Collecting(tokens) => html! {
            <ComponentBody completed=false tokens={ tokens.clone() } />
        },
        FetchState::Completed(tokens) => html! {
            <ComponentBody completed=true tokens={ tokens.clone() } />
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
    tokens: Response,
}

#[function_component(ComponentBody)]
fn component_body(props: &BodyProps) -> Html {
    let BodyProps { completed, tokens } = props;

    let content = tokens
        .choices
        .iter()
        .map(|choice| &choice.message.content)
        .join("");

    let style = if *completed { "" } else { "color: #FF3333;" };

    html! {
        <Content>
            <div { style }>
                <Markdown src={ content } />
            </div>
        </Content>
    }
}
