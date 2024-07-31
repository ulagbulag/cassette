use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaActor {
    #[serde(default)]
    pub create: Option<RootSchema>,

    #[serde(default)]
    pub update: Option<RootSchema>,
}
