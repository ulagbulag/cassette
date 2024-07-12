pub mod cassette;
pub mod error;
pub mod home;
pub mod license;
pub mod profile;

use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::build_info::{CI_PLATFORM, DEBUG, GIT_DIRTY};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub title: AttrValue,

    #[prop_or_default]
    pub experimental: bool,

    #[prop_or_default]
    pub subtitle: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageBody)]
pub fn page_body(props: &Props) -> Html {
    let Props {
        title,
        experimental,
        subtitle,
        children,
    } = props;

    let header = html! {
        <PageSection
            r#type={PageSectionType::Default}
            variant={PageSectionVariant::Light}
            limit_width=true
            sticky={[PageSectionSticky::Top]}
        >
            <Content>
                <Title size={Size::XXXXLarge}>
                    { title.clone() }
                </Title>
                { for subtitle }
            </Content>
        </PageSection>
    };

    let alert_warning = |msg| {
        html! {
            <Alert inline=true title="Warning" r#type={AlertType::Warning}>
                <p>{ msg }</p>
            </Alert>
        }
    };
    let alert_debug = || {
        alert_warning(
            "
This application is built under debugging mode.
Some features may not be completed, and you can experience significant performance degradation.
We strongly suggest not using it for production purposes.",
        )
    };
    let alert_git_dirty = || {
        alert_warning(
            "
This application was built in a dirty git environment.
It implies that this is a development build, and you may experience unexpected side effects.
We strongly suggest not using it for production purposes.",
        )
    };

    let alert_experimental = || {
        html! {
            <PageSection>
                <Alert inline=true title="Experimental feature" r#type={AlertType::Info}>
                    { Html::from_html_unchecked(r#"
<p>
This functionality is considered experimental. This means that the whole idea of it might be
changed or removed in future versions. It will also will not be considered for semantic versioning.
</p>
<p>
In order to prevent people from accidentially using such features, they are gated using Rust's
features and are not part of the <code>default</code> feature. In order to enable all experimental
features, enable the <code>experimental</code> feature. Individual experimental components can
be enabled using individual features. See the Rust docmentation for more details on which features
exist.
</p>"#.into()) }
                </Alert>
            </PageSection>
        }
    };

    let children = children
        .iter()
        .map(|child| html!(<PageSection>{child}</PageSection>));

    html! (
        <PageSectionGroup>
            { header }

            if DEBUG {
                { alert_debug() }
            } else if CI_PLATFORM.is_none() && GIT_DIRTY.unwrap_or_default() {
                { alert_git_dirty() }
            }

            if *experimental {
                { alert_experimental() }
            }

            { for children }
        </PageSectionGroup>
    )
}
