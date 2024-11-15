use cassette_core::data::actor::SchemaActor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn actor() -> SchemaActor {
    SchemaActor {
        create: None,
        update: None,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HelmList {
    #[serde(default)]
    pub chart_name: Option<String>,
    #[serde(default)]
    pub namespace: Option<String>,
}

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
