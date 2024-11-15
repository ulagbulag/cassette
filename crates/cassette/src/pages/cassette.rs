use cassette_core::{
    cassette::{Cassette as CassetteData, CassetteState},
    net::fetch::FetchState,
    prelude::*,
    task::{TaskRenderer, TaskState},
};
use patternfly_yew::prelude::*;
use tracing::info;
use uuid::Uuid;
use yew::prelude::*;

use crate::{
    components::RootCassetteTask,
    history::{History, HistoryLog},
    hooks::gateway::use_cassette,
    pages::error::ErrorKind,
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(Cassette)]
pub fn cassette(props: &Props) -> Html {
    let Props { id } = props;
    let cassette = use_cassette(*id);

    #[allow(clippy::match_single_binding)]
    match &cassette.data {
        FetchState::Pending | FetchState::Fetching => html! {
            <CassetteFallback />
        },
        FetchState::Collecting(option) | FetchState::Completed(option) => match option.as_ref() {
            Some(data) => {
                History::push(HistoryLog {
                    id: data.id,
                    name: data.name.clone(),
                });

                html! {
                    <CassetteView data={ data.clone() } />
                }
            }
            None => html! {
                <crate::pages::error::Error kind={ ErrorKind::NotFound } />
            },
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

    info!("Rendering tasks");

    let title = data.title();
    let subtitle = data.description.clone();

    let mut contents = vec![];
    {
        let trigger = use_force_update();
        let mut root_state = CassetteState::new(data.id, trigger);

        for task in data.component.tasks.iter().map(RootCassetteTask) {
            match task.render(&mut root_state) {
                Ok(TaskState::Break { body, state: _ }) => {
                    contents.push(body);
                    break;
                }
                Ok(TaskState::Continue { body, state: _ }) => {
                    contents.push(body);
                    continue;
                }
                Ok(TaskState::Skip { state: _ }) => {
                    continue;
                }
                Err(error) => {
                    let body = html! {
                        <Alert
                            inline=true
                            title="Error"
                            r#type={AlertType::Danger}
                        >
                            <p style="white-space: pre-line;">
                                { error }
                            </p>
                        </Alert>
                    };
                    contents.push(body);
                    break;
                }
            }
        }
    }

    html! {
        <super::PageBody { title } { subtitle } >
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

    let content = props
        .error
        .as_deref()
        .map(|error| {
            html! {
                <Alert
                    inline=true
                    title="Error"
                    r#type={AlertType::Danger}
                >
                    <p style="white-space: pre-line;">
                        { error }
                    </p>
                </Alert>
            }
        })
        .unwrap_or_else(|| html! { <Loading /> });

    html! {
        <super::PageBody { title } >
            { content }
        </super::PageBody>
    }
}
