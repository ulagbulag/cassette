use cassette_core::{cassette::Cassette as CassetteData, net::fetch::FetchState};
use inflector::Inflector;
use patternfly_yew::prelude::*;
use uuid::Uuid;
use yew::prelude::*;

use crate::{hooks::gateway::use_cassette, pages::error::ErrorKind};

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
    let subtitle = data.description.clone().unwrap_or_default();

    html! {
        <super::PageBody {title} {subtitle} >
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
    let title = "A Cassette";
    let subtitle = "Loading...";

    let error = match props.error.as_deref() {
        Some(error) => html! {
            <Alert inline=true title="Error" r#type={AlertType::Danger}>
                { error }
            </Alert>
        },
        None => html! {},
    };

    html! {
        <super::PageBody {title} {subtitle} >
            { error }
        </super::PageBody>
    }
}
