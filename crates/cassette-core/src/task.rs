use garde::Validate;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[cfg(feature = "ui")]
use serde_json::map::Entry;
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
    fn render(&self, state: &mut crate::cassette::CassetteState) -> TaskResult<()>;
}

#[cfg(feature = "ui")]
pub type TaskResult<T> = Result<TaskState<T>, String>;

#[cfg(feature = "ui")]
pub enum TaskState<T = Option<TaskSpec>> {
    Break { body: Html, state: T },
    Continue { body: Html, state: T },
    Skip { state: T },
}

#[cfg(feature = "ui")]
impl<T> TaskState<Option<T>>
where
    T: Serialize,
{
    pub(crate) fn try_into_spec(self) -> Result<TaskState<Option<TaskSpec>>, String> {
        let encode = |value| {
            ::serde_json::to_value(value)
                .map(TaskSpec)
                .map_err(|error| format!("Failed to encode task state: {error}"))
        };

        match self {
            Self::Break { body, state } => Ok(TaskState::Break {
                body,
                state: state.map(encode).transpose()?,
            }),
            Self::Continue { body, state } => Ok(TaskState::Continue {
                body,
                state: state.map(encode).transpose()?,
            }),
            Self::Skip { state } => Ok(TaskState::Skip {
                state: state.map(encode).transpose()?,
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct TaskSpec(pub Value);

impl Default for TaskSpec {
    fn default() -> Self {
        Self(Value::Object(Default::default()))
    }
}

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
    pub(crate) fn get(&self, key: &str) -> Result<&Value, String> {
        self.try_get(key)
            .ok_or_else(|| format!("no such key: {key}"))
    }

    pub(crate) fn try_get(&self, key: &str) -> Option<&Value> {
        match key {
            "" | "/" => Some(&self.0),
            key => self.0.pointer(key),
        }
    }

    pub(crate) fn set_child(&mut self, name: &str, value: Self) -> bool {
        let value = value.0;
        match &mut self.0 {
            Value::Null => {
                self.0 = Value::Object({
                    let mut map = ::serde_json::Map::with_capacity(1);
                    map.insert(name.into(), value);
                    map
                });
                true
            }
            Value::Object(map) => match map.entry(name) {
                Entry::Occupied(mut entry) => {
                    let entry = entry.get_mut();
                    if *entry != value {
                        *entry = value;
                        true
                    } else {
                        false
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(value);
                    true
                }
            },
            _ => false,
        }
    }
}
