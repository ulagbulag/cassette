use cassette_core::{
    cassette::CassetteContext,
    components::ComponentRenderer,
    task::{TaskResult, TaskState},
};
use patternfly_yew::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use yew::prelude::*;
use yew_markdown::Markdown;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    msg: Value,

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

        let content = match msg {
            Value::String(src) => html! { <Markdown { src } /> },
            msg => ::serde_json::to_string_pretty(&msg)
                .map(|data| {
                    html! {
                        <CodeBlock>
                            <CodeBlockCode>{ data }</CodeBlockCode>
                        </CodeBlock>
                    }
                })
                .map_err(|error| format!("Failed to encode message: {error}"))?,
        };
        let style = if progress { "color: #FF3333;" } else { "" };

        Ok(TaskState::Continue {
            body: html! {
                <Content>
                    <div { style }>
                        { content }
                    </div>
                </Content>
            },
        })
    }
}
