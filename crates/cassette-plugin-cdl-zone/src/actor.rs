use cassette_core::{
    cassette::CassetteContext,
    components::ComponentRenderer,
    task::{TaskResult, TaskState},
};
use patternfly_yew::prelude::*;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {}

impl ComponentRenderer<Spec> for State {
    fn render(self, _ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec {} = spec;

        Ok(TaskState::Continue {
            body: html! { <Manager/> },
            state: Some(Self {}),
        })
    }
}

#[function_component(Manager)]
fn manager() -> Html {
    const TOTAL_ENTRIES: usize = 394;

    let offset = use_state_eq(|| 0);
    let limit = use_state_eq(|| 5);

    let entries = use_memo((*offset, *limit), |(offset, limit)| {
        (*offset..(offset + limit).clamp(0, TOTAL_ENTRIES))
            .map(ExampleEntry)
            .collect::<Vec<_>>()
    });

    let (entries, _) = use_table_data(MemoizedTableModel::new(entries));

    let header = html_nested! {
        <TableHeader<Columns>>
            <TableColumn<Columns> index={Columns::Select} />
            <TableColumn<Columns> label="Decimal" index={Columns::Decimal} />
            <TableColumn<Columns> label="Hex" index={Columns::Hex} />
            <TableColumn<Columns> label="Button" index={Columns::Button} />
        </TableHeader<Columns>>
    };

    let total_entries = Some(TOTAL_ENTRIES);

    let limit_callback = use_callback(limit.clone(), |number, limit| limit.set(number));

    let nav_callback = use_callback(
        (offset.clone(), *limit),
        |page: Navigation, (offset, limit)| {
            let o = match page {
                Navigation::First => 0,
                Navigation::Last => ((TOTAL_ENTRIES - 1) / limit) * limit,
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
                    <ToolbarItem r#type={ToolbarItemType::Pagination}>
                        <Pagination
                            {total_entries}
                            offset={*offset}
                            entries_per_page_choices={vec![5, 10, 25, 50, 100]}
                            selected_choice={*limit}
                            onlimit={&limit_callback}
                            onnavigation={&nav_callback}
                        />
                    </ToolbarItem>
                </ToolbarContent>
            </Toolbar>
            <Table<Columns, UseTableData<Columns, MemoizedTableModel<ExampleEntry>>>
                mode={TableMode::Compact}
                {header}
                {entries}
            />
            <Pagination
                {total_entries}
                offset={*offset}
                entries_per_page_choices={vec![5, 10, 25, 50, 100]}
                selected_choice={*limit}
                onlimit={&limit_callback}
                onnavigation={&nav_callback}
                position={PaginationPosition::Bottom}
            />
        </>
    )
}

#[derive(Clone, Eq, PartialEq)]
pub enum Columns {
    Select,
    Decimal,
    Hex,
    Button,
}

#[derive(Clone)]
struct ExampleEntry(usize);

impl TableEntryRenderer<Columns> for ExampleEntry {
    fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
        match context.column {
            Columns::Select => html! { <Checkbox /> },
            Columns::Decimal => html!(self.0.to_string()),
            Columns::Hex => html!(format!("{:x}", self.0)),
            Columns::Button => html! {
                <Flex modifiers={[ FlexModifier::Shrink ]}>
                    <FlexItem>
                        <Button variant={ ButtonVariant::Primary }>{ "Describe" }</Button>
                    </FlexItem>
                    <FlexItem>
                        <Button variant={ ButtonVariant::Danger }>{ "Delete" }</Button>
                    </FlexItem>
                </Flex>
            },
        }
        .into()
    }
}
