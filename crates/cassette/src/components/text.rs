use cassette_core::{
    cassette::CassetteContext,
    component::ComponentRenderer,
    task::{TaskResult, TaskState},
};
use patternfly_yew::prelude::*;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_markdown::Markdown;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    msg: String,

    #[serde(default)]
    progress: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {}

impl ComponentRenderer<Spec> for State {
    fn render(self, _ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Self {} = self;
        let Spec { msg, progress } = spec;

        let style = if progress { "color: #FF3333;" } else { "" };

        Ok(TaskState::Continue {
            body: html! {
                <Content>
                    <div { style }>
                        <Markdown src={ msg.clone() } />
                    </div>
                </Content>
            },
        })
    }
}
