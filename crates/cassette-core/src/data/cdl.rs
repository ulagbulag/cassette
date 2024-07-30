use std::rc::Rc;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CdlTable {}

impl CdlTable {
    pub fn columns(&self) -> Vec<String> {
        todo!()
    }

    pub fn records(&self) -> Rc<Vec<Vec<Value>>> {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}
