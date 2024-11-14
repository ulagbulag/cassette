use cassette_core::{
    cassette::{CassetteTaskHandle, GenericCassetteTaskHandle},
    data::actor::SchemaPath,
};
use patternfly_yew::prelude::{ResizeOrientation, TextArea};
use serde_json::Value;
use yew::{html, Callback, Html};

pub fn build_form(handle: &CassetteTaskHandle<Value>, path: SchemaPath, disabled: bool) -> Html {
    let id = path.to_string();
    let onchange = {
        let handle = handle.clone();
        Callback::from(move |text: String| {
            if let Ok(json) = ::serde_json::from_str(&text) {
                handle.set_item(&path, json)
            }
        })
    };
    let placeholder = "{}";

    let value = match handle.get() {
        Value::Null => "{}".into(),
        value => ::serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into()),
    };

    html! {
        <TextArea
            { id }
            autofocus=true
            { disabled }
            { onchange }
            { placeholder }
            required=true
            resize={ ResizeOrientation::Vertical }
            { value }
        />
    }
}
