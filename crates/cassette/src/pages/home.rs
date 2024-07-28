use inflector::Inflector;
use itertools::Itertools;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::components::Link;

use crate::{
    history::{History, HistoryLog},
    route::AppRoute,
};

#[function_component(Home)]
pub fn home() -> Html {
    let title = env!("CARGO_PKG_NAME").to_title_case();
    let subtitle = env!("CARGO_PKG_DESCRIPTION");

    let logs = use_memo((), |()| {
        History::get()
            .into_iter()
            .rev()
            .take(10)
            .map(Entry)
            .collect_vec()
    });
    let header_log = html_nested! (
        <TableHeader<KeyColumns>>
            <TableColumn<KeyColumns> label="Name" index={ KeyColumns::Name } />
            <TableColumn<KeyColumns> label="Link" index={ KeyColumns::Link } />
        </TableHeader<KeyColumns>>
    );
    let (entries_log, _) = use_table_data(MemoizedTableModel::new(logs));

    let mode = TableMode::Compact;

    html! {
        <super::PageBody {title} {subtitle} >
            <Content>
                <h2>{ "Recently Played" }</h2>
                <Table<KeyColumns, UseTableData<KeyColumns, MemoizedTableModel<Entry>>>
                    { mode }
                    header={ header_log }
                    entries={ entries_log }
                />
            </Content>
        </super::PageBody>
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Entry(HistoryLog);

#[derive(Copy, Clone, Eq, PartialEq)]
enum KeyColumns {
    Name,
    Link,
}

impl TableEntryRenderer<KeyColumns> for Entry {
    fn render_cell(&self, ctx: CellContext<KeyColumns>) -> Cell {
        let Self(HistoryLog { id, name }) = self;
        match ctx.column {
            KeyColumns::Name => html!({ name }),
            KeyColumns::Link => html! {
                <Link<AppRoute> to={ AppRoute::Cassette { id: *id } }>
                    { name }
                </Link<AppRoute>>
            },
        }
        .into()
    }

    fn render_details(&self) -> Vec<Span> {
        vec![Span::max(html! (
            <>
                { &self.0.name }
            </>
        ))]
    }

    fn render_column_details(&self, column: &KeyColumns) -> Vec<Span> {
        vec![Span::max(match column {
            KeyColumns::Name => html!({ "Name" }),
            KeyColumns::Link => html!({ "Link" }),
        })]
    }
}
