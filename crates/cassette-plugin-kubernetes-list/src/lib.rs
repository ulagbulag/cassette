use std::marker::PhantomData;

use cassette_core::{
    cassette::CassetteState,
    task::{TaskResult, TaskSpec, TaskState},
};
use cassette_plugin_kubernetes_core::{api::Api, hooks::use_kubernetes_list};
use k8s_openapi::api::apps::v1::Deployment;
use kube_core::params::ListParams;
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

    let api: Api<Deployment> = Api {
        api_group: Some("apps".into()),
        namespace: Some("default".into()),
        plural: "deployments".into(),
        version: "v1".into(),
        _type: PhantomData,
    };
    let lp = ListParams::default();
    let list = use_kubernetes_list(api, lp);

    html! {
        <Content>
            <p>{ msg }</p>
            <p>{ format!("{list:#?}") }</p>
        </Content>
    }
}
