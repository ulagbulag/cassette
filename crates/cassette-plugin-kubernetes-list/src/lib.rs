use std::marker::PhantomData;

use cassette_core::{
    cassette::{CassetteContext, GenericCassetteTaskHandle},
    components::ComponentRenderer,
    net::fetch::FetchState,
    prelude::*,
    task::{TaskResult, TaskState},
};
use cassette_plugin_kubernetes_core::{api::Api, hooks::use_kubernetes_list};
use kube_core::{params::ListParams, DynamicObject};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    api_version: String,
    kind: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    content: ListOrItem<DynamicObject>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ListOrItem<T> {
    List(Vec<T>),
    Item(Option<T>),
}

impl<T> Default for ListOrItem<T> {
    fn default() -> Self {
        Self::Item(None)
    }
}

impl ComponentRenderer<Spec> for State {
    fn render(self, ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec { api_version, kind } = spec;

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

        match &*use_kubernetes_list(ctx, api, lp).get() {
            FetchState::Pending | FetchState::Fetching => Ok(TaskState::Break {
                body: html! { <Loading /> },
                state: None,
            }),
            FetchState::Collecting(content) | FetchState::Completed(content) => {
                Ok(TaskState::Skip {
                    state: Some(Self {
                        content: ListOrItem::List(content.items.clone()),
                    }),
                })
            }
            FetchState::Error(msg) => Ok(TaskState::Break {
                body: html! { <Error msg={ msg.clone() } /> },
                state: None,
            }),
        }
    }
}
