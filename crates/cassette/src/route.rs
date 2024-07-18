use std::{collections::BTreeSet, fmt};

use cassette_core::cassette::CassetteRef;
use inflector::Inflector;
use itertools::Itertools;
use patternfly_yew::prelude::*;
use tracing::info;
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
    Error(ErrorRoute),
    Profile,
}

impl Default for AppRoute {
    fn default() -> Self {
        Self::Error(ErrorRoute::NotFound)
    }
}

impl fmt::Display for AppRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Home => "/".fmt(f),
            Self::Cassette { id } => write!(f, "/c/{id}"),
            Self::License => "/license".fmt(f),
            Self::Error(route) => write!(f, "/error{route}"),
            Self::Profile => "/profile".fmt(f),
        }
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

        let group_names: BTreeSet<_> = cassettes
            .as_ref()
            .map(|list| {
                list.iter()
                    .filter_map(|item| item.group.as_deref())
                    .collect()
            })
            .unwrap_or_default();
        let cassettes_groups = group_names.into_iter().map(|group| {
            let cassettes = cassettes
                .as_ref()
                .ok()
                .map(|list| render_cassette_list(list, group, false));
            (group.to_string(), cassettes)
        });
        let nav_groups = cassettes_groups.map(|(group, cassettes)| {
            html! {
                <NavExpandable title={ group }>
                    { cassettes }
                </NavExpandable>
            }
        });

        html_nested! {
            <PageSidebar>
                <Nav>
                    <NavList>
                        { nav_home }
                        { nav_namespaced }
                        { for nav_groups }
                        { nav_about }
                    </NavList>
                </Nav>
            </PageSidebar>
        }
    }

    pub fn switch(self) -> Html {
        info!("Rendering app: {self}");

        let page = match self {
            Self::Home => html! { <crate::pages::home::Home /> },
            Self::Cassette { id } => html! { <crate::pages::cassette::Cassette { id } /> },
            Self::License => html! { <crate::pages::license::License /> },
            Self::Error(route) => route.switch(),
            Self::Profile => html! { <crate::pages::profile::Profile /> },
        };
        html! {
            <AppPage>
                { page }
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
        .sorted_by_key(|cassette| {
            (
                cassette.priority.unwrap_or(u32::MAX),
                &cassette.name,
                cassette.id,
            )
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
pub enum ErrorRoute {
    #[target(rename = "404")]
    NotFound,
}

impl fmt::Display for ErrorRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => "/404".fmt(f),
        }
    }
}

impl ErrorRoute {
    fn switch(self) -> Html {
        use crate::pages::error::{Error, ErrorKind};

        let kind = match self {
            Self::NotFound => ErrorKind::NotFound,
        };
        html! {
            <Error { kind } />
        }
    }
}
