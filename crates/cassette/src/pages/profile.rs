use std::borrow::Cow;

use cassette_core::net::{
    fetch::FetchState,
    gateway::{use_gateway, use_gateway_status},
};
use patternfly_yew::prelude::*;
use tracing::info;
use yew::prelude::*;
use yew_nested_router::components::Link;

use crate::{build_info::*, route::AppRoute};

#[function_component(Profile)]
pub fn profile() -> Html {
    info!("Beginning loading profile...");

    // Application
    let title = "Profile";
    let subtitle = "This page can be used to check the system or share the problem situation with experts when there is a problem.";

    let entries_app = use_memo((), |()| {
        vec![
            Entry {
                key: "Name",
                value: Cow::Borrowed(PKG_NAME),
                link: None,
            },
            Entry {
                key: "Description",
                value: Cow::Borrowed(PKG_DESCRIPTION),
                link: None,
            },
            Entry {
                key: "Authors",
                value: Cow::Borrowed(PKG_AUTHORS),
                link: None,
            },
            Entry {
                key: "Homepage",
                value: Cow::Borrowed(PKG_HOMEPAGE),
                link: Some(EntryLink::String(Cow::Borrowed(PKG_HOMEPAGE))),
            },
            Entry {
                key: "Repository",
                value: Cow::Borrowed(PKG_REPOSITORY),
                link: Some(EntryLink::String(Cow::Borrowed(PKG_REPOSITORY))),
            },
            Entry {
                key: "License",
                value: Cow::Borrowed(PKG_LICENSE),
                link: Some(EntryLink::Route(AppRoute::License)),
            },
        ]
    });
    let (entries_app, _) = use_table_data(MemoizedTableModel::new(entries_app));

    // Build
    let entries_build = use_memo((), |()| {
        vec![
            Entry {
                key: "Build CI",
                value: Cow::Borrowed(CI_PLATFORM.unwrap_or("Unknown")),
                link: None,
            },
            Entry {
                key: "Build Features",
                value: Cow::Borrowed(FEATURES_LOWERCASE_STR),
                link: None,
            },
            Entry {
                key: "Build Jobs",
                value: Cow::Owned(NUM_JOBS.to_string()),
                link: None,
            },
            Entry {
                key: "Build Profile",
                value: Cow::Borrowed(PROFILE),
                link: None,
            },
            Entry {
                key: "Build Time",
                value: Cow::Borrowed(BUILT_TIME_UTC),
                link: None,
            },
            Entry {
                key: "Build Version",
                value: Cow::Borrowed(PKG_VERSION),
                link: None,
            },
            Entry {
                key: "Debug Mode",
                value: Cow::Owned(DEBUG.to_string()),
                link: None,
            },
            Entry {
                key: "Git Commit",
                value: Cow::Borrowed(GIT_COMMIT_HASH.unwrap_or_default()),
                link: None,
            },
            Entry {
                key: "Git Dirty",
                value: Cow::Owned(GIT_DIRTY.unwrap_or_default().to_string()),
                link: None,
            },
            Entry {
                key: "Git Head ref",
                value: Cow::Borrowed(GIT_HEAD_REF.unwrap_or_default()),
                link: None,
            },
            Entry {
                key: "Git Version",
                value: Cow::Borrowed(GIT_VERSION.unwrap_or_default()),
                link: None,
            },
            Entry {
                key: "Optimization Level",
                value: Cow::Borrowed(OPT_LEVEL),
                link: None,
            },
            Entry {
                key: "Rustc",
                value: Cow::Borrowed(RUSTC),
                link: None,
            },
            Entry {
                key: "Rustc Version",
                value: Cow::Borrowed(RUSTC_VERSION),
                link: None,
            },
            Entry {
                key: "Target Arch",
                value: Cow::Borrowed(CFG_TARGET_ARCH),
                link: None,
            },
            Entry {
                key: "Target Endian",
                value: Cow::Borrowed(CFG_ENDIAN),
                link: None,
            },
            Entry {
                key: "Target Environment",
                value: Cow::Borrowed(CFG_ENV),
                link: None,
            },
            Entry {
                key: "Target OS",
                value: Cow::Borrowed(CFG_OS),
                link: None,
            },
            Entry {
                key: "Target OS Family",
                value: Cow::Borrowed(CFG_FAMILY),
                link: None,
            },
            Entry {
                key: "Target Pointer Width",
                value: Cow::Borrowed(CFG_POINTER_WIDTH),
                link: None,
            },
        ]
    });
    let (entries_build, _) = use_table_data(MemoizedTableModel::new(entries_build));

    // Dependencies
    let entries_deps = use_memo((), |()| {
        DIRECT_DEPENDENCIES
            .into_iter()
            .map(|(name, version)| Entry {
                key: name,
                value: Cow::Borrowed(version),
                link: None,
            })
            .collect()
    });
    let (entries_deps, _) = use_table_data(MemoizedTableModel::new(entries_deps));

    // Runtime
    let gateway_url = use_gateway();
    let gateway_status = use_gateway_status();

    let entries_rt = use_state_eq(|| {
        vec![
            Entry {
                key: "Gateway URL",
                value: Cow::Owned(gateway_url),
                link: None,
            },
            Entry {
                key: "Gateway Status",
                value: Cow::Owned(FetchState::<String>::Pending.to_string()),
                link: None,
            },
        ]
    });
    if let Some(index) = entries_rt
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.key == "Gateway Status")
        .map(|(index, _)| index)
    {
        entries_rt.set({
            let mut entries = (*entries_rt).clone();
            entries[index].value = Cow::Owned(gateway_status);
            entries
        });
    }

    let (entries_rt, _) = use_table_data(UseStateTableModel::new(entries_rt));

    // Table
    info!("Completed loading profile");

    let header_key = html_nested! (
        <TableHeader<KeyColumns>>
            <TableColumn<KeyColumns> label="Key" index={ KeyColumns::Key } />
            <TableColumn<KeyColumns> label="Value" index={ KeyColumns::Value } />
        </TableHeader<KeyColumns>>
    );
    let header_version = html_nested! (
        <TableHeader<VersionColumns>>
            <TableColumn<VersionColumns> label="Name" index={ VersionColumns::Name } />
            <TableColumn<VersionColumns> label="Version" index={ VersionColumns::Version } />
        </TableHeader<VersionColumns>>
    );

    let mode = TableMode::Compact;

    html! {
        <super::PageBody {title} {subtitle} >
            <Content>
                <h1>{ "System Profile" }</h1>

                <h2>{ "Application Information" }</h2>
                <Table<KeyColumns, UseTableData<KeyColumns, MemoizedTableModel<Entry>>>
                    { mode }
                    header={ header_key.clone() }
                    entries={ entries_app }
                />

                <h2>{ "Build Information" }</h2>
                <Table<KeyColumns, UseTableData<KeyColumns, MemoizedTableModel<Entry>>>
                    { mode }
                    header={ header_key.clone() }
                    entries={ entries_build }
                />

                <h2>{ "Runtime Information" }</h2>
                <Table<KeyColumns, UseTableData<KeyColumns, UseStateTableModel<Entry>>>
                    { mode }
                    header={ header_key }
                    entries={ entries_rt }
                />

                <h2>{ "Dependency Information" }</h2>
                <Table<VersionColumns, UseTableData<VersionColumns, MemoizedTableModel<Entry>>>
                    { mode }
                    header={ header_version }
                    entries={ entries_deps }
                />
            </Content>
        </super::PageBody>
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Entry<'a> {
    key: &'a str,
    value: Cow<'a, str>,
    link: Option<EntryLink<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
enum EntryLink<'a> {
    Route(AppRoute),
    String(Cow<'a, str>),
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum KeyColumns {
    Key,
    Value,
}

impl<'a> TableEntryRenderer<KeyColumns> for Entry<'a> {
    fn render_cell(&self, ctx: CellContext<KeyColumns>) -> Cell {
        let Self { key, value, link } = self;
        match ctx.column {
            KeyColumns::Key => html!({ key }),
            KeyColumns::Value => match link {
                Some(EntryLink::Route(to)) => {
                    html! {<Link<AppRoute> to={ to.clone() }>{ value }</Link<AppRoute>>}
                }
                Some(EntryLink::String(link)) => html! {<a href={ link.to_string() }>{ value }</a>},
                None => html!({ value }),
            },
        }
        .into()
    }

    fn render_details(&self) -> Vec<Span> {
        vec![Span::max(html! (
            <>
                { &self.key }
            </>
        ))]
    }

    fn render_column_details(&self, column: &KeyColumns) -> Vec<Span> {
        vec![Span::max(match column {
            KeyColumns::Key => html!({ "Key" }),
            KeyColumns::Value => html!({ "Value" }),
        })]
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum VersionColumns {
    Name,
    Version,
}

impl<'a> TableEntryRenderer<VersionColumns> for Entry<'a> {
    fn render_cell(&self, ctx: CellContext<VersionColumns>) -> Cell {
        let Self { key, value, link } = self;
        match ctx.column {
            VersionColumns::Name => match link {
                Some(EntryLink::Route(to)) => {
                    html! {<Link<AppRoute> to={ to.clone() }>{ key }</Link<AppRoute>>}
                }
                Some(EntryLink::String(link)) => html! {<a href={ link.to_string() }>{ key }</a>},
                None => html!({ key }),
            },
            VersionColumns::Version => html!({ value }),
        }
        .into()
    }

    fn render_details(&self) -> Vec<Span> {
        vec![Span::max(html! (
            <>
                { &self.key }
            </>
        ))]
    }

    fn render_column_details(&self, column: &VersionColumns) -> Vec<Span> {
        vec![Span::max(match column {
            VersionColumns::Name => html!({ "Name" }),
            VersionColumns::Version => html!({ "Version" }),
        })]
    }
}
