use kube::CustomResource;
use schemars::JsonSchema;
#[cfg(feature = "ui")]
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::task::CassetteTask;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, CustomResource)]
#[kube(
    group = "cassette.ulagbulag.io",
    version = "v1alpha1",
    kind = "CassetteComponent",
    root = "CassetteComponentCrd",
    shortname = "casc",
    namespaced,
    printcolumn = r#"{
        "name": "created-at",
        "type": "date",
        "description": "created time",
        "jsonPath": ".metadata.creationTimestamp"
    }"#,
    printcolumn = r#"{
        "name": "version",
        "type": "integer",
        "description": "component version",
        "jsonPath": ".metadata.generation"
    }"#
)]
#[serde(rename_all = "camelCase")]
pub struct CassetteComponentSpec {
    #[serde(default)]
    pub tasks: Vec<CassetteTask>,
}

#[cfg(feature = "ui")]
pub trait ComponentRenderer<Spec> {
    fn render(
        self,
        ctx: &mut crate::cassette::CassetteContext,
        spec: Spec,
    ) -> crate::task::TaskResult<Option<Self>>
    where
        Self: Sized;
}

#[cfg(feature = "ui")]
pub trait ComponentRendererExt<Spec>
where
    Self: Default + Serialize + DeserializeOwned + ComponentRenderer<Spec>,
    Spec: DeserializeOwned,
{
    fn render_with(
        mut ctx: crate::cassette::CassetteContext,
        spec: &crate::task::TaskSpec,
    ) -> crate::task::TaskResult<()>
    where
        Self: Sized,
    {
        use serde_json::Value;

        fn replace_key(
            ctx: &crate::cassette::CassetteContext,
            spec: &crate::task::TaskSpec,
            value: &Value,
        ) -> Result<Value, String> {
            match value {
                Value::Null => Ok(Value::Null),
                Value::Bool(data) => Ok(Value::Bool(*data)),
                Value::Number(data) => Ok(Value::Number(data.clone())),
                Value::String(data) => {
                    if data.starts_with(":/") {
                        ctx.get(&data[1..]).cloned()
                    } else if data.starts_with("~/") {
                        spec.get(&data[1..]).cloned()
                    } else if data.starts_with("\\:/") || data.starts_with("\\~/") {
                        Ok(Value::String(data[1..].into()))
                    } else {
                        Ok(Value::String(data.clone()))
                    }
                }
                Value::Array(array) => array
                    .iter()
                    .map(|value| replace_key(ctx, spec, value))
                    .collect::<Result<_, _>>()
                    .map(Value::Array),
                Value::Object(map) => map
                    .iter()
                    .map(|(key, value)| {
                        replace_key(ctx, spec, value).map(|value| (key.clone(), value))
                    })
                    .collect::<Result<_, _>>()
                    .map(Value::Object),
            }
        }

        let state = ctx.get_task_state()?.unwrap_or_default();

        let spec = replace_key(&ctx, spec, &spec.0)?;
        let spec = ::serde_json::from_value(spec)
            .map_err(|error| format!("Failed to parse task spec: {error}"))?;

        let state = <Self as ComponentRenderer<Spec>>::render(state, &mut ctx, spec)
            .and_then(crate::task::TaskState::try_into_spec)?;
        Ok(ctx.set(state))
    }
}

#[cfg(feature = "ui")]
impl<Spec, T> ComponentRendererExt<Spec> for T
where
    Self: Default + Serialize + DeserializeOwned + ComponentRenderer<Spec>,
    Spec: DeserializeOwned,
{
}
