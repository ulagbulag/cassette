use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SchemaSpec {
    #[serde(default)]
    pub choices: Vec<String>,
    #[serde(default)]
    pub default: Option<String>,
}
