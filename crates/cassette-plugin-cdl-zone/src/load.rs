use std::rc::Rc;

use cassette_core::cassette::GenericCassetteTaskHandle;
use cassette_core::prelude::*;
use cassette_core::{
    cassette::CassetteContext,
    components::ComponentRenderer,
    data::table::DataTable,
    net::fetch::FetchState,
    task::{TaskResult, TaskState},
};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[serde(default)]
    base_url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    #[serde(default, flatten)]
    data: Option<Rc<DataTable>>,
}

impl ComponentRenderer<Spec> for State {
    fn render(self, ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec { base_url } = spec;

        let force_init = false;

        match crate::hooks::use_fetch(ctx, base_url, force_init).get() {
            FetchState::Pending | FetchState::Fetching => Ok(TaskState::Break {
                body: html! { <Loading /> },
                state: Some(Self { data: None }),
            }),
            FetchState::Collecting(content) | FetchState::Completed(content) => {
                Ok(TaskState::Skip {
                    state: Some(Self {
                        data: Some((**content).clone()),
                    }),
                })
            }
            FetchState::Error(msg) => Ok(TaskState::Break {
                body: html! { <Error msg={ msg.clone() } /> },
                state: Some(Self { data: None }),
            }),
        }
    }
}
