use std::{
    borrow::Borrow,
    cmp,
    hash::{Hash, Hasher},
};

use garde::Validate;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, Validate, CustomResource)]
#[kube(
    group = "cassette.ulagbulag.io",
    version = "v1alpha1",
    kind = "Cassette",
    root = "CassetteCrd",
    shortname = "cas",
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
        "description": "cassette version",
        "jsonPath": ".metadata.generation"
    }"#
)]
#[serde(rename_all = "camelCase")]
pub struct CassetteSpec {
    #[garde(length(min = 1, max = 253), pattern("^[0-9][0-9-]*[0-9]*$"))]
    #[serde(default)]
    pub component: String,
    #[garde(length(min = 1, max = 1024))]
    #[serde(default)]
    pub description: Option<String>,
    #[garde(length(min = 1, max = 1024))]
    #[serde(default)]
    pub group: Option<String>,
    #[garde(skip)]
    #[serde(default)]
    pub priority: Option<u32>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CassetteRef<Component = Uuid> {
    pub id: Uuid,
    pub component: Component,
    pub name: String,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub priority: Option<u32>,
}

impl<ComponentId> PartialEq for CassetteRef<ComponentId>
where
    ComponentId: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<ComponentId> Eq for CassetteRef<ComponentId> where ComponentId: Eq {}

impl<ComponentId> PartialOrd for CassetteRef<ComponentId>
where
    ComponentId: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<ComponentId> Ord for CassetteRef<ComponentId>
where
    ComponentId: Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<ComponentId> Hash for CassetteRef<ComponentId>
where
    ComponentId: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<ComponentId> Borrow<Uuid> for CassetteRef<ComponentId> {
    fn borrow(&self) -> &Uuid {
        &self.id
    }
}
