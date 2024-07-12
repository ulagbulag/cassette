use cassette_core::net::fetch::FetchState;
use inflector::Inflector;
use patternfly_yew::prelude::{Switch as ToggleSwitch, *};
use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};

use crate::{
    hooks::{
        gateway::use_cassette_list,
        redirect::{use_open, OpenTarget},
    },
    route::AppRoute,
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BackdropViewer>
                <ToastViewer>
                    <Router<AppRoute> default={AppRoute::default()}>
                        <RouterSwitch<AppRoute> render={AppRoute::switch} />
                    </Router<AppRoute>>
                </ToastViewer>
            </BackdropViewer>
        </main>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AppPageProps {
    pub children: Children,
}

#[function_component(AppPage)]
pub fn app_page(props: &AppPageProps) -> Html {
    let AppPageProps { children } = props;

    let cassette_list = use_cassette_list();
    let cassette_list = match &*cassette_list {
        FetchState::Pending | FetchState::Fetching => Err(html! { <p>{ "Loading..." }</p> }),
        FetchState::Completed(list) => Ok(list.as_slice()),
        FetchState::Error(error) => Err(html! { <p>{ format!("Error: {error}") }</p> }),
    };

    let sidebar = AppRoute::use_side_bar(cassette_list);

    let title = env!("CARGO_PKG_NAME")
        .to_screaming_snake_case()
        .replace('_', " ");
    let brand = html! (
        <MastheadBrand>
            <div style="display: flex; align-items: center; justify-content: space-between; gap: 0.5rem; user-select: none; color: #DEDEDE;">
                <Brand src="assets/images/icons/logo.webp" alt="Main Logo" style="--pf-v5-c-brand--Height: 36px;"/>
                <Title size={Size::XLarge} >
                    { title }
                </Title>
            </div>
        </MastheadBrand>
    );

    let callback_github = use_open(env!("CARGO_PKG_REPOSITORY"), OpenTarget::Blank);

    // track dark mode state
    let darkmode = use_state_eq(|| {
        ::gloo_utils::window()
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .map(|m| m.matches())
            .unwrap_or_default()
    });

    // apply dark mode
    use_effect_with(*darkmode, |state| match state {
        true => ::gloo_utils::document_element().set_class_name("pf-v5-theme-dark"),
        false => ::gloo_utils::document_element().set_class_name(""),
    });

    // toggle dark mode
    let onthemeswitch = use_callback(darkmode.setter(), |state, setter| setter.set(state));

    let tools = html!(
        <Toolbar full_height=true>
            <ToolbarContent>
                <ToolbarGroup
                    modifiers={ToolbarElementModifier::Right.all()}
                    variant={GroupVariant::IconButton}
                >
                    <ToolbarItem>
                        <ToggleSwitch checked={*darkmode} onchange={onthemeswitch} label="Dark Theme" />
                    </ToolbarItem>
                    <ToolbarItem>
                        <Button variant={ButtonVariant::Plain} icon={Icon::Github} onclick={callback_github}/>
                    </ToolbarItem>
                </ToolbarGroup>
            </ToolbarContent>
        </Toolbar>
    );

    html! {
        <Page {brand} {sidebar} {tools}>
            { for children.iter() }
        </Page>
    }
}
