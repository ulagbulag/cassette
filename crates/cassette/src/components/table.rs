use cassette_core::{
    cassette::CassetteContext,
    components::ComponentRenderer,
    data::table::DataTable,
    task::{TaskResult, TaskState},
};
use patternfly_yew::prelude::*;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    table: DataTable,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {}

impl ComponentRenderer<Spec> for State {
    fn render(self, _ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec { table: _ } = spec;

        // TODO: to be implemented
        Ok(TaskState::Continue {
            body: html! {
                <Content>
                </Content>
            },
            state: None,
        })
    }
}
