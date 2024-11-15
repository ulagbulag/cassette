use cassette_core::{
    cassette::CassetteTaskHandle,
    data::actor::{string::SchemaSpec, SchemaPath},
};
use patternfly_yew::prelude::{TextInputGroup, TextInputGroupMain};
use serde_json::Value;
use yew::{html, Callback, Html};

pub fn build_form(
    handle: &CassetteTaskHandle<Value>,
    name: String,
    path: SchemaPath,
    spec: SchemaSpec,
    default: &Value,
    disabled: bool,
) -> Html {
    let id = path.to_string();
    let onchange = {
        let handle = handle.clone();
        Callback::from(move |text: String| handle.set_item(&path, Value::String(text)))
    };

    let value = match default {
        Value::String(value) => value.clone(),
        value => spec
            .default
            .clone()
            .or_else(|| ::serde_json::to_string_pretty(value).ok())
            .unwrap_or_default(),
    };

    html! {
        <div style="margin-bottom: 16px;">
            { name }
            <TextInputGroup>
                <TextInputGroupMain
                    { id }
                    { disabled }
                    { onchange }
                    placeholder={ spec.default }
                    { value }
                />
            </TextInputGroup>
        </div>
    }
}
