use cassette_core::data::actor::JsonSchemaActor;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn actor() -> JsonSchemaActor {
    JsonSchemaActor {
        create: Some(schema_for!(HelmPut)),
        update: Some(schema_for!(HelmPost)),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HelmDelete {
    pub name: String,
    #[serde(default)]
    pub namespace: Option<String>,
}

pub type HelmDeleteOutput = String;

pub type HelmPost = HelmPut;

pub type HelmPostOutput = String;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HelmPut {
    pub chart_name: String,
    pub name: String,
    #[serde(default)]
    pub namespace: Option<String>,
    pub repo: String,
    #[serde(default)]
    pub values: Value,
}

pub type HelmPutOutput = String;
