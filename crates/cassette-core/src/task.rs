use garde::Validate;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, Validate)]
#[schemars(bound = "Spec: Default + JsonSchema")]
#[serde(
    bound = "Spec: Default + Serialize + DeserializeOwned",
    rename_all = "camelCase"
)]
pub struct CassetteTask<Spec = Map<String, Value>> {
    #[garde(length(min = 1, max = 253), pattern("^[0-9][0-9-]*[0-9]*$"))]
    pub name: String,
    #[garde(length(min = 1, max = 253), pattern("^[0-9][0-9-]*[0-9]*$"))]
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
