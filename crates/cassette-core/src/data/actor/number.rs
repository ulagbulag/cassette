use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaSpec {
    #[serde(default)]
    pub default: Option<Number>,
}
