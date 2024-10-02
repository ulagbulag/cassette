use cassette_core::{
    cassette::{CassetteContext, GenericCassetteTaskHandle},
    components::ComponentRenderer,
    prelude::*,
    task::{TaskResult, TaskState},
};
use cassette_plugin_webcam_core::{hooks::use_webcam, Constraints, Handler};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[serde(flatten)]
    handler: Handler,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {}

impl ComponentRenderer<Spec> for State {
    fn render(self, ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec { handler } = spec;

        let constraints = Constraints {
            audio: true,
            video: false,
        };
        let webcam = match use_webcam(ctx, &handler, &constraints).get() {
            Ok(webcam) => webcam,
            Err(msg) => {
                return Ok(TaskState::Break {
                    body: html! { <Error msg={ msg.clone() } /> },
                    state: None,
                })
            }
        };

        Ok(TaskState::Skip {
            state: Some(Self {}),
        })
    }
}
