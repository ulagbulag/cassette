use cassette_core::{
    cassette::{CassetteTaskHandle, GenericCassetteTaskHandle},
    data::actor::{Schema, SchemaArray, SchemaType},
};
use patternfly_yew::prelude::FormGroup;
use serde_json::Value;
use yew::{html, Html};

pub fn build_form(handle: &CassetteTaskHandle<Value>, spec: SchemaArray, disabled: bool) -> Html {
    let default = handle.get();
    let children = spec.0.into_iter().map(|Schema { name, path, ty }| {
        let default = path.get(default);
        match ty {
            SchemaType::Boolean(spec) => {
                super::boolean::build_form(handle, name, path, spec, default, disabled)
            }
            SchemaType::Number(spec) => {
                super::number::build_form(handle, name, path, spec, default, disabled)
            }
            SchemaType::String(spec) => {
                super::string::build_form(handle, name, path, spec, default, disabled)
            }
            SchemaType::Enum(spec) => {
                super::r#enum::build_form(handle, name, path, spec, default, disabled)
            }
        }
    });

    html! {
        <FormGroup>
            { for children }
        </FormGroup>
    }
}
