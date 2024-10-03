#[cfg(feature = "ui")]
pub mod hooks;
#[cfg(feature = "ui")]
pub mod recorder;

use serde::{Deserialize, Serialize};
#[cfg(feature = "ui")]
use yew::Properties;

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ui", derive(Properties))]
#[serde(rename_all = "camelCase")]
pub struct Constraints {
    pub audio: bool,
    pub video: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ui", derive(Properties))]
#[serde(rename_all = "camelCase")]
pub struct Handler {
    #[serde(default)]
    pub duration: Option<u32>,
    #[serde(default = "Handler::default_interval")]
    pub interval: u32,
    pub url: String,
}

impl Default for Handler {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            interval: Self::default_interval(),
            url: Default::default(),
        }
    }
}

impl Handler {
    const fn default_interval() -> u32 {
        20 // 20 ms
    }
}
