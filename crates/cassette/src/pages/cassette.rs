use patternfly_yew::prelude::*;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(Cassette)]
pub fn cassette(props: &Props) -> Html {
    let Props { id } = props;
    let _ = id;

    #[allow(clippy::match_single_binding)]
    match () {
        () => html! { <CassetteFallback /> },
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
