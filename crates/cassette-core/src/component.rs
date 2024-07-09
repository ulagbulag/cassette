use kube::CustomResource;
use schemars::JsonSchema;
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
