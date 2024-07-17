use garde::Validate;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "ui")]
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, Validate)]
#[schemars(bound = "Spec: Default + JsonSchema")]
#[serde(
    bound = "Spec: Default + Serialize + DeserializeOwned",
    rename_all = "camelCase"
)]
pub struct CassetteTask<Spec = TaskSpec> {
    #[garde(length(min = 1, max = 253), pattern("^[a-z][a-z0-9-]*[a-z0-9]*$"))]
    pub name: String,
    #[garde(length(min = 1, max = 253), pattern("^[A-Z][A-Za-z0-9-]*$"))]
    pub kind: String,
    #[garde(skip)]
    #[serde(default)]
    pub metadata: CassetteTaskMetadata,
    #[garde(skip)]
    #[serde(default)]
    #[schemars(schema_with = "TaskSpec::preserve_arbitrary")]
    pub spec: Spec,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CassetteTaskMetadata {
    #[serde(default)]
    pub column: CassetteTaskColumnType,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
)]
pub enum CassetteTaskColumnType {
    All,
    #[default]
    Current,
    New,
}

#[cfg(feature = "ui")]
pub trait TaskRenderer {
    fn render(&self, state: &UseStateHandle<crate::cassette::CassetteState>) -> TaskResult;
}

#[cfg(feature = "ui")]
pub type TaskResult<T = TaskState> = Result<T, String>;

#[cfg(feature = "ui")]
pub enum TaskState {
    Break { body: Html },
    Continue { body: Html },
    Skip,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct TaskSpec(Value);

impl TaskSpec {
    fn preserve_arbitrary(
        _gen: &mut ::schemars::gen::SchemaGenerator,
    ) -> ::schemars::schema::Schema {
        let mut obj = ::schemars::schema::SchemaObject::default();
        obj.extensions
            .insert("x-kubernetes-preserve-unknown-fields".into(), true.into());
        ::schemars::schema::Schema::Object(obj)
    }
}

#[cfg(feature = "ui")]
impl TaskSpec {
    fn get(&self, key: &str) -> TaskResult<&Value> {
        match key {
            "" | "/" => Ok(&self.0),
            key => self
                .0
                .pointer(key)
                .ok_or_else(|| format!("no such key: {key}")),
        }
    }

    pub fn get_string(&self, key: &str) -> TaskResult<String> {
        self.get(key).and_then(|value| match value {
            Value::String(value) => Ok(value.clone()),
            _ => Err(format!("value is not a string: {key}")),
        })
    }

    pub fn get_model<T>(&self, key: &str) -> TaskResult<T>
    where
        T: DeserializeOwned,
    {
        self.get(key).and_then(|value| {
            ::serde_json::from_value(value.clone())
                .map_err(|error| format!("failed to parse value: {key}: {error}"))
        })
    }
}
