use cassette_core::{
    cassette::CassetteTaskHandle,
    data::actor::{number::SchemaSpec, SchemaPath},
};
use patternfly_yew::prelude::{TextInputGroup, TextInputGroupMain};
use serde_json::{Number, Value};
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
        Callback::from(move |updated_text: String| {
            if let Ok(number) = updated_text.parse() {
                handle.set_item(&path, Value::Number(number))
            }
        })
    };

    let build_default = || Number::from(0 as u8);
    let value = match default {
        Value::Bool(value) => Number::from(*value as u8),
        Value::Number(value) => value.clone(),
        Value::String(value) => value.parse().unwrap_or_else(|_| build_default()),
        _ => spec.default.clone().unwrap_or_else(build_default),
    };

    html! {
        <>
            { name }
            <TextInputGroup style="padding: 4px; margin-bottom: 8px;">
                <TextInputGroupMain
                    { id }
                    { disabled }
                    { onchange }
                    placeholder={ spec.default.as_ref().map(ToString::to_string) }
                    value={ value.to_string() }
                />
            </TextInputGroup>
        </>
    }
}
