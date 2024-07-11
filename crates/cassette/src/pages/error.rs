use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub kind: ErrorKind,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    NotFound,
}

impl ErrorKind {
    fn as_str(&self) -> &str {
        match self {
            Self::NotFound => "Not Found :/",
        }
    }

    // fn code(&self) -> u16 {
    //     match self {
    //         Self::NotFound => 404,
    //     }
    // }
}

#[function_component(Error)]
pub fn error(props: &Props) -> Html {
    html! {
        <PageSectionGroup>
            <PageSection>
                <Alert inline=true title="Error" r#type={AlertType::Danger}>
                    { props.kind.as_str() }
                </Alert>
            </PageSection>
        </PageSectionGroup>
    }
}
