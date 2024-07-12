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

use crate::{component::CassetteComponentSpec, task::TaskSpec};

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
    #[garde(length(min = 1, max = 253), pattern("^[a-z][a-z0-9-]*[a-z0-9]*$"))]
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

pub type CassetteRef = Cassette<Uuid>;

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Cassette<Component = CassetteComponentSpec> {
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

impl<Component> PartialEq for Cassette<Component>
where
    Component: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<Component> Eq for Cassette<Component> where Component: Eq {}

impl<Component> PartialOrd for Cassette<Component>
where
    Component: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<Component> Ord for Cassette<Component>
where
    Component: Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<Component> Hash for Cassette<Component>
where
    Component: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<Component> Borrow<Uuid> for Cassette<Component> {
    fn borrow(&self) -> &Uuid {
        &self.id
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CassetteState {
    cassette: Cassette,
    data: TaskSpec,
}

impl CassetteState {
    pub fn new(cassette: Cassette) -> Self {
        Self {
            cassette,
            data: TaskSpec::default(),
        }
    }
}
