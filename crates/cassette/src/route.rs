use patternfly_yew::prelude::*;
use uuid::Uuid;
use yew::{prelude::*, virtual_dom::VChild};
use yew_nested_router::prelude::*;

use crate::app::AppPage;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[target(rename = "")]
    Home,
    #[target(rename = "c")]
    Cassette {
        id: Uuid,
    },
    License,
    Error(Errors),
    Profile,
}

impl Default for AppRoute {
    fn default() -> Self {
        Self::Error(Errors::NotFound)
    }
}

impl AppRoute {
    pub fn side_bar() -> VChild<PageSidebar> {
        let nav_home = html! {
            <NavExpandable title="Basics">
                <NavRouterItem<AppRoute> to={AppRoute::Home}>{"Home"}</NavRouterItem<AppRoute>>
            </NavExpandable>
        };

        let nav_about = html! {
            <NavExpandable title="About">
                <NavRouterItem<AppRoute> to={AppRoute::Profile}>{"Profile"}</NavRouterItem<AppRoute>>
                <NavRouterItem<AppRoute> to={AppRoute::License}>{"License"}</NavRouterItem<AppRoute>>
            </NavExpandable>
        };

        html_nested! {
            <PageSidebar>
                <Nav>
                    <NavList>
                        {nav_home}
                        {nav_about}
                    </NavList>
                </Nav>
            </PageSidebar>
        }
    }

    pub fn switch(self) -> Html {
        let page = match self {
            Self::Home => html! { <crate::pages::home::Home /> },
            Self::Cassette { id } => html! { <crate::pages::home::Home /> },
            Self::License => html! { <crate::pages::license::License /> },
            Self::Error(route) => route.switch(),
            Self::Profile => html! { <crate::pages::profile::Profile /> },
        };
        html! {
            <AppPage>
                {page}
            </AppPage>
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Target)]
pub enum Errors {
    #[target(rename = "404")]
    NotFound,
}

impl Errors {
    fn switch(self) -> Html {
        use crate::pages::error::{Error, ErrorKind};

        let kind = match self {
            Self::NotFound => ErrorKind::NotFound,
        };
        html! {
            <Error {kind} />
        }
    }
}
