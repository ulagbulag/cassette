use std::io::Read;

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CsvTable {
    pub records: Vec<Value>,
}

impl CsvTable {
    pub(super) fn from_reader(reader: impl Read) -> Result<Self> {
        ::csv::Reader::from_reader(reader)
            .into_deserialize()
            .map(|record| record.map_err(Into::into))
            .collect::<Result<_>>()
            .map(|records| Self { records })
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
}
