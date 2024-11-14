use cassette_core::{
    cassette::CassetteTaskHandle,
    data::actor::{SchemaArray, SchemaPath},
};
use serde_json::Value;
use yew::Html;

pub fn build_form(
    handle: &CassetteTaskHandle<Value>,
    schema: Option<SchemaArray>,
    disabled: bool,
) -> Html {
    match schema {
        Some(spec) => super::array::build_form(handle, spec, disabled),
        None => {
            let path = SchemaPath::default();
            super::generic::build_form(handle, path, disabled)
        }
    }
}
