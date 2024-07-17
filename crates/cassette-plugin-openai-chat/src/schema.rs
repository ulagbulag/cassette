use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Request {
    #[serde(default = "Request::default_model")]
    pub model: String,
    #[serde(default, flatten)]
    pub options: RequestOptions,
    #[serde(default)]
    pub messages: Vec<Message>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            model: Self::default_model(),
            options: Default::default(),
            messages: Default::default(),
        }
    }
}

impl Request {
    fn default_model() -> String {
        "any".into()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestOptions {
    #[serde(default = "RequestOptions::default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default)]
    pub stream: Option<bool>,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            max_tokens: Self::default_max_tokens(),
            stream: Default::default(),
        }
    }
}

impl RequestOptions {
    const fn default_max_tokens() -> u32 {
        1000
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageChoice {
    #[serde(default)]
    pub index: u32,
    #[serde(alias = "delta")]
    pub message: Message,
    #[serde(default)]
    pub finish_reason: Option<MessageFinishReason>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageFinishReason {
    EosToken,
    Length,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Assistant,
    User,
    System,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Response {
    pub choices: VecDeque<MessageChoice>,
}
