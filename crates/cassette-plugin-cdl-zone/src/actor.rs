use std::rc::Rc;

use cassette_core::{
    cassette::CassetteContext,
    components::ComponentRenderer,
    data::table::DataTable,
    prelude::*,
    task::{TaskResult, TaskState},
};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    values: Rc<DataTable>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {}

impl ComponentRenderer<Spec> for State {
    fn render(self, _ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec { values } = spec;

        Ok(TaskState::Break {
            body: html! {
                <BaseActor uri="/cdl/zone" { values } >
                </BaseActor>
            },
            state: Some(Self {}),
        })
    }
}
