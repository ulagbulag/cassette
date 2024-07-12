use cassette_core::{
    cassette::{Cassette as CassetteData, CassetteState},
    net::fetch::FetchState,
    task::{TaskRenderer, TaskState},
};
use inflector::Inflector;
use patternfly_yew::prelude::*;
use uuid::Uuid;
use yew::prelude::*;

use crate::{components::RootCassetteTask, hooks::gateway::use_cassette, pages::error::ErrorKind};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(Cassette)]
pub fn cassette(props: &Props) -> Html {
    let Props { id } = props;
    let cassette = use_cassette(*id);

    #[allow(clippy::match_single_binding)]
    match &*cassette {
        FetchState::Pending | FetchState::Fetching => html! {
            <CassetteFallback />
        },
        FetchState::Completed(Some(data)) => html! {
            <CassetteView data={ data.clone() } />
        },
        FetchState::Completed(None) => html! {
            <crate::pages::error::Error kind={ ErrorKind::NotFound } />
        },
        FetchState::Error(error) => html! {
            <CassetteFallback error={ Some(error.clone()) } />
        },
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct DataProps {
    pub data: CassetteData,
}

#[function_component(CassetteView)]
fn cassette_data(props: &DataProps) -> Html {
    let DataProps { data } = props;

    let title = data.name.to_title_case();
    let subtitle = data.description.clone();

    let state = use_state(|| CassetteState::new(data.clone()));
    let mut contents = vec![];
    for task in data.component.tasks.iter().map(RootCassetteTask) {
        match task.render(&state) {
            Ok(TaskState::Break { body }) => {
                contents.push(body);
                break;
            }
            Ok(TaskState::Continue { body }) => {
                contents.push(body);
                continue;
            }
            Ok(TaskState::Skip) => {
                continue;
            }
            Err(error) => {
                let body = html! {
                    <Alert inline=true title="Error" r#type={AlertType::Danger}>
                        { error }
                    </Alert>
                };
                contents.push(body);
                break;
            }
        }
    }

    html! {
        <super::PageBody {title} {subtitle} >
            { for contents }
        </super::PageBody>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct FallbackProps {
    #[prop_or_default]
    pub error: Option<String>,
}

#[function_component(CassetteFallback)]
fn cassette_fallback(props: &FallbackProps) -> Html {
    let FallbackProps { error } = props;

    let title = if error.is_some() { "Error" } else { "" };
    let subtitle = if error.is_some() {
        None
    } else {
        Some("Loading...")
    };

    let error = props.error.as_deref().map(|error| {
        html! {
            <Alert inline=true title="Error" r#type={AlertType::Danger}>
                { error }
            </Alert>
        }
    });

    html! {
        <super::PageBody {title} {subtitle} >
            { for error }
        </super::PageBody>
    }
}
