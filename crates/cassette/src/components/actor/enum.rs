use cassette_core::{
    cassette::CassetteTaskHandle,
    data::actor::{r#enum::SchemaSpec, SchemaPath},
};
use patternfly_yew::prelude::{FormSelect, FormSelectOption};
use serde_json::Value;
use yew::{html, html_nested, Callback, Html};

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
        Callback::from(move |text: Option<String>| {
            let text = match text {
                Some(text) => Value::String(text),
                None => Value::Null,
            };
            handle.set_item(&path, text)
        })
    };

    let choices: Vec<_> = spec
        .choices
        .into_iter()
        .map(|value| {
            let id = format!("{id}/{value}");
            html_nested! {
                <FormSelectOption<String>
                    { id }
                    { value }
                />
            }
        })
        .collect();

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
            <FormSelect<String>
                { id }
                { disabled }
                { onchange }
                placeholder={ spec.default }
                { value }
            >
                { for choices }
            </FormSelect<String>>
        </div>
    }
}
