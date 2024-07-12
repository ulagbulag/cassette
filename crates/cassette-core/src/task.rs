use garde::Validate;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Map, Value};
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
pub struct TaskSpec(Map<String, Value>);

impl TaskSpec {
    pub fn get_string(&self, key: &str) -> TaskResult<String> {
        self.0
            .get(&key[1..])
            .ok_or_else(|| format!("no such key: {key}"))
            .and_then(|value| match value {
                Value::String(value) => Ok(value.clone()),
                _ => Err(format!("value is not a string: {key}")),
            })
    }
}
