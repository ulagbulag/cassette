mod array;
mod boolean;
mod generic;
mod number;
mod root;
mod string;

use std::{borrow::Cow, rc::Rc};

use cassette_core::{
    cassette::{CassetteContext, CassetteTaskHandle, GenericCassetteTaskHandle},
    components::ComponentRenderer,
    data::{
        actor::{SchemaActor, SchemaArray},
        table::DataTable,
    },
    net::{
        fetch::{Body, FetchRequest, FetchRequestWithoutBody, FetchState, Method},
        gateway::get_gateway,
    },
    prelude::*,
    task::{TaskResult, TaskState},
};
use patternfly_yew::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[serde(default)]
    pub base_url: Option<String>,
    pub uri: String,
    #[serde(default)]
    pub schema: Rc<SchemaActor>,
    #[serde(default)]
    pub default: Value,
    #[serde(default)]
    pub table: Option<Rc<DataTable>>,

    #[serde(default = "Spec::default_label_create")]
    pub label_create: String,
    #[serde(default = "Spec::default_label_delete")]
    pub label_delete: String,
    #[serde(default = "Spec::default_label_update")]
    pub label_update: String,

    #[serde(default = "Spec::default_create")]
    pub create: bool,

    #[serde(default = "Spec::default_delete")]
    pub delete: bool,

    #[serde(default = "Spec::default_update")]
    pub update: bool,
}

impl Spec {
    fn default_label_create() -> String {
        "Create".into()
    }

    fn default_label_delete() -> String {
        "Delete".into()
    }

    fn default_label_update() -> String {
        "Update".into()
    }

    const fn default_create() -> bool {
        true
    }

    const fn default_delete() -> bool {
        true
    }

    const fn default_update() -> bool {
        true
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {}

impl ComponentRenderer<Spec> for State {
    fn render(self, ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let Spec {
            base_url,
            uri,
            schema,
            default,
            table,
            label_create,
            label_delete,
            label_update,
            create,
            delete,
            update,
        } = spec;

        let force_init = false;

        let content = match use_fetch_actor(ctx, base_url.clone(), &uri, force_init).get() {
            FetchState::Pending | FetchState::Fetching => {
                return Ok(TaskState::Break {
                    body: html! { <Loading /> },
                    state: None,
                })
            }
            FetchState::Collecting(content) | FetchState::Completed(content) => content.clone(),
            FetchState::Error(msg) => {
                return Ok(TaskState::Break {
                    body: html! { <Error msg={ msg.clone() } /> },
                    state: Some(self),
                })
            }
        };

        let mut sections = vec![];
        let mut tabs = vec![];

        // NOTE: Ordered
        if create
            && table
                .as_ref()
                .map(|table| table.data.len() == 0)
                .unwrap_or(true)
        {
            sections.push(build_form(FormContext {
                ctx,
                base_url: base_url.as_ref(),
                uri: &uri,
                schema: content.create.as_ref(),
                schema_additional: schema.create.as_ref(),
                default: &default,
                table: None,
                label_submit: label_create.clone(),
            }));
            tabs.push(TabIndex::Create);
        }
        if update
            && table
                .as_ref()
                .map(|table| table.data.len() == 1)
                .unwrap_or_default()
        {
            // TODO: load data

            sections.push(build_form(FormContext {
                ctx,
                base_url: base_url.as_ref(),
                uri: &uri,
                schema: content.update.as_ref().or(content.create.as_ref()),
                schema_additional: schema.create.as_ref(),
                default: &default,
                table: None,
                label_submit: label_create.clone(),
            }));
            tabs.push(TabIndex::Update);
        }
        if delete
            && table
                .as_ref()
                .map(|table| table.data.len() >= 1)
                .unwrap_or_default()
        {
            sections.push(build_form_delete(FormDeleteContext {
                ctx,
                base_url: base_url.as_ref(),
                uri: &uri,
                table: table.clone().unwrap(),
                label_apply: label_delete.clone(),
            }));
            tabs.push(TabIndex::Delete);
        }

        let index_title = |index| match index {
            TabIndex::Create => label_create.clone(),
            TabIndex::Delete => label_delete.clone(),
            TabIndex::Update => label_update.clone(),
        };

        match tabs.len() {
            0 => Ok(TaskState::Break {
                body: html! {
                    <Content>
                        { "No available actions" }
                    </Content>
                },
                state: Some(self),
            }),
            1 => Ok(TaskState::Break {
                body: {
                    let title = tabs.into_iter().map(|index| {
                        html! {
                            <h2>{ index_title(index) }</h2>
                        }
                    });
                    html! {
                        <Content>
                            { for title }
                            { for sections }
                        </Content>
                    }
                },
                state: Some(self),
            }),
            2.. => {
                let handler_name = "tab";
                let selected_tab = ctx.use_state(handler_name, force_init, || {
                    tabs.first().copied().unwrap_or_default()
                });
                let onselect_tab = {
                    let selected_tab = selected_tab.clone();
                    Callback::from(move |index| selected_tab.set(index))
                };

                let sections = sections
                    .into_iter()
                    .zip(tabs.clone())
                    .map(|(section, index)| {
                        html! {
                            <section hidden={ *selected_tab.get() != index }>{ section }</section>
                        }
                    });

                let tabs = tabs.into_iter().map(|index| {
                    let title = index_title(index);
                    html_nested! {
                        <Tab<TabIndex> { index } { title } />
                    }
                });

                Ok(TaskState::Break {
                    body: html! {
                        <>
                            <Tabs<TabIndex>
                                r#box=true
                                detached=true
                                onselect={ onselect_tab }
                                selected={ *selected_tab.get() }
                            >
                                { for tabs }
                            </Tabs<TabIndex>>

                            { for sections }
                        </>
                    },
                    state: Some(self),
                })
            }
        }
    }
}

struct FormContext<'a, 'b> {
    ctx: &'a mut CassetteContext<'b>,
    base_url: Option<&'a String>,
    uri: &'a String,
    schema: Option<&'a SchemaArray>,
    schema_additional: Option<&'a SchemaArray>,
    default: &'a Value,
    table: Option<Rc<DataTable>>,
    label_submit: String,
}

fn build_form(ctx: FormContext) -> Html {
    let FormContext {
        ctx,
        base_url,
        uri,
        schema,
        schema_additional,
        default,
        table,
        label_submit,
    } = ctx;

    let is_post = table.is_some();
    let handler_name_prefix = if is_post { "form post" } else { "form put" };

    let handler_name_submit = format!("{handler_name_prefix} submit");
    let force_init = false;
    let submit_state = ctx.use_state(&handler_name_submit, force_init, || {
        FetchState::<Value>::Pending
    });

    let handler_name_data = format!("{handler_name_prefix} data");
    let handle_data = ctx.use_state(handler_name_data, force_init, || default.clone());
    let disabled = matches!(
        submit_state.get(),
        FetchState::Fetching | FetchState::Collecting(_)
    );
    let schema = match (schema, schema_additional) {
        (Some(schema), Some(schema_additional)) => {
            let mut schema = schema.clone();
            schema.0.extend_from_slice(&schema_additional.0);
            Some(schema)
        }
        (Some(schema), None) | (None, Some(schema)) => Some(schema.clone()),
        (None, None) => None,
    };
    let form_data = self::root::build_form(&handle_data, schema, disabled);

    let onclick = {
        let base_url = base_url.cloned();
        let uri = uri.clone();
        let submit_state = submit_state.clone();
        let handle_data = handle_data.clone();
        Callback::from(move |_: MouseEvent| {
            let base_url = base_url.clone();
            let handler_name = handler_name_submit.clone();
            let uri = uri.clone();

            let state = submit_state.clone();
            let base_url = base_url.unwrap_or(get_gateway());
            let request = FetchRequest {
                method: if is_post { Method::POST } else { Method::PUT },
                name: Cow::Owned(handler_name),
                uri,
                body: Some(Body::Json(handle_data.get().clone())),
            };

            request.try_fetch_force(&base_url, state)
        })
    };

    let output_success = |msg| {
        html! {
            <Content>
                <Alert inline=true title="Success" r#type={ AlertType::Success }>
                    { msg }
                </Alert>
            </Content>
        }
    };
    let output = match submit_state.get() {
        FetchState::Pending => Html::default(),
        FetchState::Fetching | FetchState::Collecting(_) => html! {
            <Loading />
        },
        FetchState::Completed(body) => match &**body {
            Value::Null => output_success("Completed!".to_string()),
            Value::String(msg) => output_success(msg.clone()),
            body => match ::serde_json::to_string_pretty(body) {
                Ok(msg) => output_success(msg),
                Err(msg) => html! {
                    <Error msg={ msg.to_string() } />
                },
            },
        },
        FetchState::Error(msg) => html! {
            <Error msg={ msg.clone() } />
        },
    };

    let button_variant = if is_post {
        ButtonVariant::Warning
    } else {
        ButtonVariant::Primary
    };

    html! {
        <Stack gutter=true>
            <StackItem>
                { form_data }
            </StackItem>
            <StackItem>
                <Button
                    { disabled }
                    { onclick }
                    variant={ button_variant }
                >
                    { label_submit }
                </Button>
            </StackItem>
            { output }
        </Stack>
    }
}

struct FormDeleteContext<'a, 'b> {
    ctx: &'a mut CassetteContext<'b>,
    base_url: Option<&'a String>,
    uri: &'a String,
    #[allow(dead_code)]
    table: Rc<DataTable>,
    label_apply: String,
}

fn build_form_delete(ctx: FormDeleteContext) -> Html {
    let FormDeleteContext {
        ctx,
        base_url,
        uri,
        table: _,
        label_apply,
    } = ctx;

    let handler_name = "form delete apply";
    let force_init = false;
    let state = ctx.use_state(handler_name, force_init, || FetchState::<Value>::Pending);

    let onclick = {
        let base_url = base_url.cloned();
        let uri = uri.clone();
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            let state = state.clone();
            let base_url = base_url.clone().unwrap_or(get_gateway());
            let request = FetchRequest {
                method: Method::POST,
                name: Cow::Borrowed(handler_name),
                uri: uri.clone(),
                body: Some(Body::Json(Value::default())),
            };

            request.try_fetch_force(&base_url, state)
        })
    };

    let output_success = |msg| {
        // TODO: use toast instead, and force-reload
        html! {
            <Content>
                <Alert inline=true title="Success" r#type={ AlertType::Success }>
                    { msg }
                </Alert>
            </Content>
        }
    };
    let output = match state.get() {
        FetchState::Pending => Html::default(),
        FetchState::Fetching | FetchState::Collecting(_) => html! {
            <Loading />
        },
        FetchState::Completed(body) => match &**body {
            Value::Null => output_success("Completed!".to_string()),
            Value::String(msg) => output_success(msg.clone()),
            body => match ::serde_json::to_string_pretty(body) {
                Ok(msg) => output_success(msg),
                Err(msg) => html! {
                    <Error msg={ msg.to_string() } />
                },
            },
        },
        FetchState::Error(msg) => html! {
            <Error msg={ msg.clone() } />
        },
    };

    let disabled = matches!(
        state.get(),
        FetchState::Fetching | FetchState::Collecting(_)
    );

    html! {
        <Stack gutter=true>
            <StackItem>
                <Button
                    { disabled }
                    { onclick }
                    variant={ ButtonVariant::Danger }
                >
                    { label_apply }
                </Button>
            </StackItem>
            { output }
        </Stack>
    }
}

fn use_fetch_actor(
    ctx: &mut CassetteContext,
    base_url: Option<String>,
    uri: &str,
    force: bool,
) -> CassetteTaskHandle<FetchState<SchemaActor>> {
    let handler_name = "fetch";
    let state = ctx.use_state(handler_name, force, || FetchState::Pending);
    {
        let state = state.clone();
        let base_url = base_url.unwrap_or(get_gateway());
        let request = FetchRequestWithoutBody {
            method: Method::GET,
            name: Cow::Borrowed(handler_name),
            uri: format!("{uri}/_actor"),
            body: None,
        };

        request.try_fetch(&base_url, state)
    }
    state
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum TabIndex {
    #[default]
    Create,
    Delete,
    Update,
}
