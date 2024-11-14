use std::borrow::Cow;
use std::collections::BTreeMap;
use std::rc::Rc;

use cassette_core::cassette::{CassetteTaskHandle, GenericCassetteTaskHandle};
use cassette_core::net::fetch::{FetchRequestWithoutBody, Method};
use cassette_core::net::gateway::get_gateway;
use cassette_core::prelude::*;
use cassette_core::{
    cassette::CassetteContext,
    components::ComponentRenderer,
    data::table::DataTable,
    net::fetch::FetchState,
    task::{TaskResult, TaskState},
};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[serde(default)]
    base_url: Option<String>,
    uri: String,
    #[serde(default)]
    query: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    #[serde(default, flatten)]
    data: Option<Rc<DataTable>>,
}

impl ComponentRenderer<Spec> for State {
    fn render(self, ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec {
            base_url,
            uri,
            query,
        } = spec;

        let force_init = false;

        match use_fetch(ctx, base_url, uri, query, force_init).get() {
            FetchState::Pending | FetchState::Fetching => Ok(TaskState::Break {
                body: html! { <Loading /> },
                state: Some(Self { data: None }),
            }),
            FetchState::Collecting(content) | FetchState::Completed(content) => {
                Ok(TaskState::Skip {
                    state: Some(Self {
                        data: Some((*content).clone()),
                    }),
                })
            }
            FetchState::Error(msg) => Ok(TaskState::Break {
                body: html! { <Error msg={ msg.clone() } /> },
                state: Some(Self { data: None }),
            }),
        }
    }
}

fn use_fetch(
    ctx: &mut CassetteContext,
    base_url: Option<String>,
    mut uri: String,
    query: BTreeMap<String, String>,
    force: bool,
) -> CassetteTaskHandle<FetchState<DataTable>> {
    let handler_name = "fetch";
    let state = ctx.use_state(handler_name, force, || FetchState::Pending);
    {
        let state = state.clone();
        let base_url = base_url.unwrap_or(get_gateway());
        let request = FetchRequestWithoutBody {
            method: Method::GET,
            name: Cow::Borrowed(handler_name),
            uri: if query.is_empty() {
                uri
            } else {
                uri.push('?');
                for (index, (key, value)) in query.iter().enumerate() {
                    if index > 0 {
                        uri.push('&');
                    }
                    uri.push_str(key);
                    uri.push('=');
                    uri.push_str(value);
                }
                uri
            },
            body: None,
        };

        request.try_fetch(&base_url, state)
    }
    state
}
