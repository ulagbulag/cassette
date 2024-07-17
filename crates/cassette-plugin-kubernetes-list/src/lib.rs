use std::{fmt, marker::PhantomData};

use cassette_core::{
    cassette::CassetteState,
    net::fetch::FetchState,
    task::{TaskResult, TaskSpec, TaskState},
};
use cassette_plugin_kubernetes_core::{api::Api, hooks::use_kubernetes_list};
use kube_core::{params::ListParams, DynamicObject};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub fn render(_state: &UseStateHandle<CassetteState>, spec: &TaskSpec) -> TaskResult {
    let api_version = spec.get_string("/apiVersion")?;
    let kind = spec.get_string("/kind")?;

    Ok(TaskState::Continue {
        body: html! { <Component { api_version } { kind } /> },
    })
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct Props {
    api_version: String,
    kind: String,
}

#[function_component(Component)]
fn component(props: &Props) -> Html {
    let Props { api_version, kind } = props;

    // TODO: to be implemented
    let _ = (api_version, kind);

    let api: Api<DynamicObject> = Api {
        api_group: Some("apps".into()),
        namespace: Some("default".into()),
        plural: "deployments".into(),
        version: "v1".into(),
        _type: PhantomData,
    };
    let lp = ListParams::default();
    let value = use_kubernetes_list(api, lp);

    match &*value {
        FetchState::Pending | FetchState::Fetching => html! {
            <Content>
                <p>{ "Loading..." }</p>
            </Content>
        },
        FetchState::Completed(data) => html! {
            <ComponentBody<DynamicObject> list={ data.items.clone() } />
        },
        FetchState::Error(error) => html! {
            <Alert inline=true title="Error" r#type={AlertType::Danger}>
                { error.clone() }
            </Alert>
        },
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct BodyProps<T>
where
    T: PartialEq,
{
    list: Vec<T>,
}

#[function_component(ComponentBody)]
fn component_body<T>(props: &BodyProps<T>) -> Html
where
    T: fmt::Debug + PartialEq,
{
    let BodyProps { list } = props;

    html! {
        <CodeBlock>
            <CodeBlockCode>
                { format!("{list:#?}") }
            </CodeBlockCode>
        </CodeBlock>
    }
}
