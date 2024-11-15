use cassette_core::{
    cassette::CassetteTaskHandle,
    data::actor::{boolean::SchemaSpec, SchemaPath},
};
use patternfly_yew::prelude::Switch;
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
        Callback::from(move |state: bool| handle.set_item(&path, Value::Bool(state)))
    };

    let checked = match default {
        Value::Bool(value) => *value,
        Value::String(value) => value.parse().unwrap_or_default(),
        _ => spec.default.unwrap_or_default(),
    };

    html! {
        <div style="margin-bottom: 16px;">
            <Switch
                { id }
                label={ name }
                { disabled }
                { onchange }
                { checked }
            />
        </div>
    }
}
