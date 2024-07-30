use std::rc::Rc;

use cassette_core::cassette::CassetteTaskHandle;
use cassette_core::data::csv::CsvTable;
use cassette_core::data::table::{DataTableLog, DataTableSource};
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
pub struct State {
    #[serde(default, flatten)]
    table: Option<Rc<DataTable>>,
}

impl ComponentRenderer<Spec> for State {
    fn render(self, ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec {
            table: DataTable { name, data, log },
        } = spec;

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

        if records.is_empty() {
            return Ok(TaskState::Break {
                body: html!("empty"),
                state: None,
            });
        }

        let handler_name = "select";
        let force_init = false;
        let num_records = records.len();
        let selections = ctx.use_state(handler_name, force_init, || vec![false; num_records]);

        let selected: Vec<_> = selections
            .iter()
            .enumerate()
            .filter(|(_, selected)| **selected)
            .filter_map(|(index, _)| records.get(index).cloned())
            .collect();

        let body = html! {
            <Inner
                columns={ columns.clone() }
                { log }
                name={ name.clone() }
                { records }
                { selections }
            />
        };

        if selected.is_empty() {
            Ok(TaskState::Break { body, state: None })
        } else {
            Ok(TaskState::Continue {
                body,
                state: Some(Self {
                    table: Some(Rc::new(DataTable {
                        name,
                        data: Rc::new(DataTableSource::Csv(CsvTable {
                            headers: columns,
                            records: Rc::new(selected),
                        })),
                        log,
                    })),
                }),
            })
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct Props {
    columns: Vec<String>,
    log: DataTableLog,
    name: String,
    records: Rc<Vec<Vec<Value>>>,
    selections: CassetteTaskHandle<Vec<bool>>,
}

#[function_component(Inner)]
fn inner(props: &Props) -> Html {
    let Props {
        columns,
        log: _,
        name,
        records,
        selections,
    } = props;

    let header = Column::build_headers(columns, selections);

    let offset = use_state_eq(|| 0);
    let limit = use_state_eq(|| 5);
    let entries_per_page_choices = vec![5, 10, 25, 50, 100];

    let total_entries = records.len();
    let entries = use_memo((*offset, *limit), |(offset, limit)| {
        records[*offset..(offset + limit).clamp(0, total_entries)]
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, values)| Entry {
                index: offset + index,
                values,
            })
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
                    <ToolbarItem r#type={ ToolbarItemType::BulkSelect }>
                        { name }
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
enum Column {
    Select {
        selections: CassetteTaskHandle<Vec<bool>>,
    },
    Value {
        index: usize,
        key: String,
    },
}

impl Column {
    fn build_headers(
        columns: &[String],
        selections: &CassetteTaskHandle<Vec<bool>>,
    ) -> VChild<TableHeader<Self>> {
        let columns = columns.iter().enumerate().map(|(index, key)| {
            html_nested! {
                <TableColumn<Self>
                    index={ Self::Value {
                        index,
                        key: key.clone(),
                    } }
                    label={ key.clone() }
                />
            }
        });

        html_nested! {
            <TableHeader<Self>>
                <TableColumn<Self>
                    index={ Self::Select {
                        selections: selections.clone(),
                    } }
                />
                { for columns }
            </TableHeader<Self>>
        }
    }
}

#[derive(Clone)]
struct Entry {
    index: usize,
    values: Vec<Value>,
}

impl TableEntryRenderer<Column> for Entry {
    fn render_cell(&self, context: CellContext<'_, Column>) -> Cell {
        match context.column {
            Column::Select { selections } => html! {
                <Checkbox
                    checked={ match selections.get_item(self.index) {
                        Some(true) => CheckboxState::Checked,
                        Some(false) => CheckboxState::Unchecked,
                        None => CheckboxState::Indeterminate,
                    } }
                    onchange={
                        let index = self.index;
                        let selections = selections.clone();
                        Callback::from(move |state: CheckboxState| selections.set_item(index, state.into()))
                    }
                />
            },
            Column::Value { index, key: _ } => self
                .values
                .get(*index)
                .map(|value| match value {
                    Value::Null => Html::default(),
                    Value::String(value) => html!(value.clone()),
                    value => html!(value.to_string()),
                })
                .unwrap_or_default(),
        }
        .into()
    }
}
