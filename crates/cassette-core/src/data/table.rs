use anyhow::{bail, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DataTable<Data = DataTableSource> {
    pub name: String,
    pub data: Data,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "data", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataTableSource {
    Cdl(super::cdl::CdlTable),
    Csv(super::csv::CsvTable),
    Raw(Vec<u8>),
}

impl DataTableSource {
    pub fn is_empty(&self) -> bool {
        match self {
            DataTableSource::Cdl(data) => data.is_empty(),
            DataTableSource::Csv(data) => data.is_empty(),
            DataTableSource::Raw(data) => data.is_empty(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
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
