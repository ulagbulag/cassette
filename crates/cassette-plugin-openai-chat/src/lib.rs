mod hooks;
mod schema;

use cassette_core::{
    cassette::{CassetteContext, GenericCassetteTaskHandle},
    components::ComponentRenderer,
    net::fetch::FetchState,
    prelude::*,
    task::{TaskResult, TaskState},
};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::schema::{Message, Request, Role};

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    base_url: String,

    #[serde(default)]
    message: Option<String>,

    #[serde(default, flatten)]
    request: Request,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    content: Option<String>,
    progress: bool,
}

impl ComponentRenderer<Spec> for State {
    fn render(self, ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec {
            base_url,
            message,
            request,
        } = spec;

        let mut request = request.clone();
        if let Some(content) = message {
            request.messages.push(Message {
                role: Role::User,
                content,
            });
        }

        match &*crate::hooks::use_fetch(ctx, &base_url, request).get() {
            FetchState::Pending | FetchState::Fetching => Ok(TaskState::Break {
                body: html! { <Loading /> },
                state: None,
            }),
            FetchState::Collecting(content) => Ok(TaskState::Skip {
                state: Some(Self {
                    content: Some((**content).clone()),
                    progress: true,
                }),
            }),
            FetchState::Completed(content) => Ok(TaskState::Skip {
                state: Some(Self {
                    content: Some((**content).clone()),
                    progress: false,
                }),
            }),
            FetchState::Error(msg) => Ok(TaskState::Break {
                body: html! { <Error msg={ msg.clone() } /> },
                state: None,
            }),
        }
    }
}
