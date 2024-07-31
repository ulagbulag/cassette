use cassette_core::{
    cassette::{CassetteContext, GenericCassetteTaskHandle},
    components::ComponentRenderer,
    keycode::KEYCODE_ENTER,
    task::{TaskResult, TaskState},
};
use patternfly_yew::prelude::*;
use serde::{Deserialize, Serialize};
use yew::{html::IntoPropValue, prelude::*};

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[serde(default)]
    default: Option<String>,
    #[serde(default)]
    label: Option<String>,
    #[serde(default = "Spec::default_label_submit")]
    label_submit: String,
    #[serde(default)]
    placeholder: Option<String>,
}

impl Spec {
    fn default_label_submit() -> String {
        "Submit".into()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    text: Option<String>,
}

impl ComponentRenderer<Spec> for State {
    fn render(self, ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec {
            default,
            label,
            label_submit,
            placeholder,
        } = spec;

        let handler_name = "text";
        let force_init = false;
        let text = ctx
            .use_state(handler_name, force_init, || default.unwrap_or_default())
            .lazy();

        let onchange = {
            let text = text.clone();
            Callback::from(move |updated_text: String| {
                if text.get().as_str() != updated_text.as_str() {
                    text.set(updated_text)
                }
            })
        };
        let onkeydown = {
            let text = text.clone();
            Callback::from(move |e: KeyboardEvent| {
                if e.key_code() == KEYCODE_ENTER {
                    e.prevent_default();
                    text.trigger()
                }
            })
        };
        let onclick = {
            let text = text.clone();
            Callback::from(move |_: MouseEvent| text.trigger())
        };

        let label = label.map(|label| html! { <Content>{ label }</Content> });
        let body = html! {
            <>
                { label }
                <TextInputGroup style="padding: 4px;">
                    <TextInputGroupMain style="margin-right: 4px;"
                        autofocus=true
                        { onchange }
                        { onkeydown }
                        { placeholder }
                        value={ text.clone() }
                    />
                    <Button { onclick } variant={ ButtonVariant::Primary }>{ label_submit }</Button>
                </TextInputGroup>
            </>
        };

        if text.get().is_empty() {
            Ok(TaskState::Break {
                body,
                state: Some(Self { text: None }),
            })
        } else {
            Ok(TaskState::Continue {
                body,
                state: Some(Self {
                    text: Some(text.into_prop_value()),
                }),
            })
        }
    }
}
