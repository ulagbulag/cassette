use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaSpec {
    #[serde(default)]
    pub default: Option<String>,
}
