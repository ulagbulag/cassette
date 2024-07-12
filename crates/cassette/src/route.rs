use cassette_core::cassette::CassetteRef;
use inflector::Inflector;
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
    #[hook]
    pub fn use_side_bar(cassettes: Result<&[CassetteRef], Html>) -> VChild<PageSidebar> {
        let cassettes_home = cassettes
            .as_ref()
            .map(|list| render_cassette_list(list, "home", false));
        let nav_home = html! {
            <NavExpandable title={ env!("CARGO_PKG_NAME").to_title_case() }>
                <NavRouterItem<AppRoute> to={AppRoute::Home}>{"Home"}</NavRouterItem<AppRoute>>
                { for cassettes_home }
            </NavExpandable>
        };

        let cassettes_namespaced = match cassettes.as_ref() {
            Ok(cassettes) => {
                if cassettes.is_empty() {
                    None
                } else {
                    Some(render_cassette_list(cassettes, "my-cassettes", true))
                }
            }
            Err(error) => Some(render_cassette_fallback(error)),
        };
        let nav_namespaced = cassettes_namespaced.map(|cassettes| {
            html! {
                <NavExpandable title="My Cassettes">
                    { cassettes }
                </NavExpandable>
            }
        });

        let cassettes_about = cassettes
            .as_ref()
            .map(|list| render_cassette_list(list, "about", false));
        let nav_about = html! {
            <NavExpandable title="About">
                <NavRouterItem<AppRoute> to={AppRoute::Profile}>{"Profile"}</NavRouterItem<AppRoute>>
                <NavRouterItem<AppRoute> to={AppRoute::License}>{"License"}</NavRouterItem<AppRoute>>
                    { for cassettes_about }
            </NavExpandable>
        };

        html_nested! {
            <PageSidebar>
                <Nav>
                    <NavList>
                        { nav_home }
                        { nav_namespaced }
                        { nav_about }
                    </NavList>
                </Nav>
            </PageSidebar>
        }
    }

    pub fn switch(self) -> Html {
        let page = match self {
            Self::Home => html! { <crate::pages::home::Home /> },
            Self::Cassette { id } => html! { <crate::pages::cassette::Cassette {id} /> },
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

fn render_cassette_list(cassettes: &[CassetteRef], group: &str, is_default: bool) -> Html {
    let items = cassettes
        .iter()
        .filter(|cassette| {
            cassette
                .group
                .as_deref()
                .map(|name| name == group)
                .unwrap_or(is_default)
        })
        .map(render_cassette);
    html! { for items }
}

fn render_cassette(cassette: &CassetteRef) -> Html {
    let CassetteRef {
        id,
        component: _,
        name,
        group: _,
        description: _,
        priority: _,
    } = cassette;

    let id = *id;
    let name = name.to_title_case();

    html! {
        <NavRouterItem<AppRoute> to={AppRoute::Cassette { id }}>{ name }</NavRouterItem<AppRoute>>
    }
}

fn render_cassette_fallback(child: &Html) -> Html {
    child.clone()
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
