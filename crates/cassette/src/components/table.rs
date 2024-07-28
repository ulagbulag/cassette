use cassette_core::data::table::DataTableLog;
use cassette_core::prelude::*;
use cassette_core::{
    cassette::CassetteContext,
    components::ComponentRenderer,
    data::table::DataTable,
    task::{TaskResult, TaskState},
};
use patternfly_yew::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use yew::prelude::*;
use yew::virtual_dom::VChild;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    table: DataTable,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {}

impl ComponentRenderer<Spec> for State {
    fn render(self, _ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec { table } = spec;

        let data = table.data;
        let log = table.log;

        let columns = match data.columns() {
            Ok(columns) => columns,
            Err(error) => {
                return Ok(TaskState::Break {
                    body: html! { <Error msg={ error.to_string() } /> },
                    state: None,
                })
            }
        };
        let records = match data.records() {
            Ok(records) => records,
            Err(error) => {
                return Ok(TaskState::Break {
                    body: html! { <Error msg={ error.to_string() } /> },
                    state: None,
                })
            }
        };

        Ok(TaskState::Continue {
            body: html! {
                <Inner
                    { columns }
                    { log }
                    { records }
                />
            },
            state: None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct Props {
    columns: Vec<String>,
    log: DataTableLog,
    records: Vec<Vec<Value>>,
}

#[function_component(Inner)]
fn inner(props: &Props) -> Html {
    let Props {
        columns,
        log: _,
        records,
    } = props;

    let header = Column::build_headers(columns);

    let offset = use_state_eq(|| 0);
    let limit = use_state_eq(|| 5);
    let entries_per_page_choices = vec![5, 10, 25, 50, 100];

    let total_entries = records.len();
    let entries = use_memo((*offset, *limit), |(offset, limit)| {
        records[*offset..(offset + limit).clamp(0, total_entries)]
            .into_iter()
            .cloned()
            .map(Entry)
            .collect()
    });
    let (entries, _) = use_table_data(MemoizedTableModel::new(entries));

    let limit_callback = use_callback(limit.clone(), |number, limit| limit.set(number));
    let nav_callback = use_callback(
        (offset.clone(), *limit),
        move |page: Navigation, (offset, limit)| {
            let o = match page {
                Navigation::First => 0,
                Navigation::Last => ((total_entries - 1) / limit) * limit,
                Navigation::Previous => **offset - limit,
                Navigation::Next => **offset + limit,
                Navigation::Page(n) => n * limit,
            };
            offset.set(o);
        },
    );

    html! (
        <>
            <Toolbar>
                <ToolbarContent>
                    // FIXME: add bulk-select support: https://www.patternfly.org/components/table/react-demos/bulk-select/
                    <ToolbarItem r#type={ ToolbarItemType::Pagination }>
                        <Pagination
                            { total_entries }
                            offset={ *offset }
                            entries_per_page_choices={ entries_per_page_choices.clone() }
                            selected_choice={ *limit }
                            onlimit={ &limit_callback }
                            onnavigation={ &nav_callback }
                        />
                    </ToolbarItem>
                </ToolbarContent>
            </Toolbar>
            <Table<Column, UseTableData<Column, MemoizedTableModel<Entry>>>
                mode={ TableMode::Compact }
                { header }
                { entries }
            />
            <Pagination
                { total_entries }
                offset={ *offset }
                entries_per_page_choices={ entries_per_page_choices }
                selected_choice={ *limit }
                onlimit={ &limit_callback }
                onnavigation={ &nav_callback }
                position={ PaginationPosition::Bottom }
            />
        </>
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Column {
    index: usize,
    key: String,
}

impl Column {
    fn build_headers(columns: &[String]) -> VChild<TableHeader<Self>> {
        let columns = columns.into_iter().enumerate().map(|(index, key)| {
            html_nested! {
                <TableColumn<Self>
                    label={ key.clone() }
                    index={ Self {
                        index,
                        key: key.clone(),
                    } }
                />
            }
        });

        html_nested! {
            <TableHeader<Self>>
                { for columns }
            </TableHeader<Self>>
        }
    }
}

#[derive(Clone)]
struct Entry(Vec<Value>);

impl TableEntryRenderer<Column> for Entry {
    fn render_cell(&self, context: CellContext<'_, Column>) -> Cell {
        self.0
            .get(context.column.index)
            .map(|value| match value {
                Value::Null => Html::default(),
                Value::String(value) => html!(value.clone()),
                value => html!(value.to_string()),
            })
            .unwrap_or_default()
            .into()
    }
}
