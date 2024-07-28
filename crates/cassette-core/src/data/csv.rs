use std::io::Read;

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CsvTable {
    pub headers: Vec<String>,
    pub records: Vec<Vec<Value>>,
}

impl CsvTable {
    pub(super) fn from_reader(reader: impl Read) -> Result<Self> {
        let mut reader = ::csv::Reader::from_reader(reader);

        let headers = reader.headers()?.deserialize(None)?;
        let records = reader
            .into_deserialize()
            .map(|record| record.map_err(Into::into))
            .collect::<Result<_>>()?;

        Ok(Self { headers, records })
    }

    pub fn columns(&self) -> Vec<String> {
        self.headers.clone()
    }

    pub fn records(self) -> Vec<Vec<Value>> {
        self.records
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
}
