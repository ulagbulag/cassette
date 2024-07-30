use std::rc::Rc;

use anyhow::{bail, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DataTable<Data = Rc<DataTableSource>> {
    pub name: String,
    pub data: Data,
    #[serde(default)]
    pub log: DataTableLog,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DataTableLog {
    id: Uuid,
    version: u64,
}

impl Default for DataTableLog {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            version: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "data", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataTableSource {
    #[cfg(feature = "cdl")]
    Cdl(super::cdl::CdlTable),
    Csv(super::csv::CsvTable),
    Raw(Vec<u8>),
}

impl DataTableSource {
    pub fn columns(&self) -> Result<Vec<String>> {
        match self {
            #[cfg(feature = "cdl")]
            DataTableSource::Cdl(data) => Ok(data.columns()),
            DataTableSource::Csv(data) => Ok(data.columns()),
            DataTableSource::Raw(_) => bail!("Raw data table has no columns"),
        }
    }

    pub fn records(&self) -> Result<Rc<Vec<Vec<Value>>>> {
        match self {
            #[cfg(feature = "cdl")]
            DataTableSource::Cdl(data) => Ok(data.records()),
            DataTableSource::Csv(data) => Ok(data.records()),
            DataTableSource::Raw(_) => bail!("Raw data table has no records"),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            #[cfg(feature = "cdl")]
            DataTableSource::Cdl(data) => data.is_empty(),
            DataTableSource::Csv(data) => data.is_empty(),
            DataTableSource::Raw(data) => data.is_empty(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            #[cfg(feature = "cdl")]
            DataTableSource::Cdl(data) => data.len(),
            DataTableSource::Csv(data) => data.len(),
            DataTableSource::Raw(data) => data.len(),
        }
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    EnumString,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "kebab-case")]
pub enum DataTableSourceType {
    Cdl,
    Csv,
    Raw,
}

impl DataTableSourceType {
    pub fn parse_bytes(&self, bytes: Vec<u8>) -> Result<DataTableSource> {
        match self {
            Self::Cdl => bail!("Unsupported data type: {self}"),
            Self::Csv => {
                super::csv::CsvTable::from_reader(bytes.as_slice()).map(DataTableSource::Csv)
            }
            Self::Raw => Ok(DataTableSource::Raw(bytes)),
        }
    }
}
