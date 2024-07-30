use std::rc::Rc;

use byte_unit::{Byte, UnitType};
use cassette_core::{
    cassette::{CassetteContext, CassetteTaskHandle, GenericCassetteTaskHandle},
    components::ComponentRenderer,
    data::table::{DataTable, DataTableLog, DataTableSourceType},
    net::fetch::FetchState,
    prelude::*,
    task::{TaskResult, TaskState},
};
use futures::AsyncReadExt;
use patternfly_yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_streams::ReadableStream;
use web_sys::{FileList, HtmlElement, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use yew_hooks::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[serde(default = "Spec::default_label_title")]
    label_title: String,

    #[serde(default)]
    label_detail: Option<String>,

    #[serde(default = "Spec::default_type")]
    r#type: DataTableSourceType,
}

impl Spec {
    fn default_label_title() -> String {
        "Upload".into()
    }

    const fn default_type() -> DataTableSourceType {
        DataTableSourceType::Raw
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    #[serde(default, flatten)]
    file: Option<Rc<DataTable>>,
}

impl ComponentRenderer<Spec> for State {
    fn render(self, ctx: &mut CassetteContext, spec: Spec) -> TaskResult<Option<Self>> {
        let file = ctx.use_state("file", false, || FetchState::Pending);

        let body = html! {
            <Manager
                drop_content={ file.clone() }
                { spec }
            />
        };

        match file.get() {
            FetchState::Completed(file) => Ok(TaskState::Continue {
                body,
                state: Some(Self {
                    file: Some(file.clone()),
                }),
            }),
            FetchState::Pending
            | FetchState::Fetching
            | FetchState::Collecting(_)
            | FetchState::Error(_) => Ok(TaskState::Break { body, state: None }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct Props {
    drop_content: CassetteTaskHandle<FetchState<DataTable>>,
    spec: Spec,
}

#[function_component(Manager)]
fn manager(props: &Props) -> Html {
    let Props {
        drop_content,
        spec:
            Spec {
                label_title,
                label_detail,
                r#type,
            },
    } = props;
    let r#type = *r#type;

    let label_detail = label_detail
        .clone()
        .unwrap_or_else(|| format!("Please upload a .{} file", r#type));

    let node = use_node_ref();

    let drop = use_drop_with_options(
        node.clone(),
        UseDropOptions {
            onfiles: {
                let drop_content = drop_content.clone();
                Some(Box::new(move |files, _data_transfer| {
                    drop_content.set_files(r#type, files)
                }))
            },
            ..Default::default()
        },
    );

    let file_input_ref = use_node_ref();

    let onchange = {
        let drop_content = drop_content.clone();
        Callback::from(move |event: Event| {
            let input: HtmlInputElement = event.target_unchecked_into();
            if let Some(files) = input.files() {
                drop_content.set_file_list(r#type, files)
            }
        })
    };
    let onclick = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(element) = file_input_ref.cast::<HtmlElement>() {
                element.click();
            }
        })
    };
    let ondrop = {
        let drop_content = drop_content.clone();
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            if let Some(files) = event.data_transfer().and_then(|dt| dt.files()) {
                drop_content.set_file_list(r#type, files)
            }
        })
    };

    let (disabled, footer) = match drop_content.get() {
        FetchState::Pending => (false, None),
        FetchState::Fetching | FetchState::Collecting(_) => (
            true,
            Some(html! {
                <p style="color: grey;">
                    { "Loading..." }
                </p>
            }),
        ),
        FetchState::Completed(file) => {
            let data_size = match r#type {
                DataTableSourceType::Raw => Byte::from(file.data.len())
                    .get_appropriate_unit(UnitType::Decimal)
                    .to_string(),
                _ => file.data.len().to_string(),
            };
            (
                false,
                Some(html! {
                    <>
                        <i class={ Icon::File.as_classes() }/>
                        { " " }
                        { file.name.clone() }
                        { " (" }
                        { data_size }
                        { ")" }
                    </>
                }),
            )
        }
        FetchState::Error(error) => (false, Some(html! { <Error msg={ error.clone() } /> })),
    };
    let footer = footer.map(|content| html! { <CardFooter>{ content }</CardFooter> });

    html! {
        <div
            ref={ node.clone() }
            { disabled }
            { onclick }
            { ondrop }
        >
            <input // placeholder
                ref={ file_input_ref }
                type="file"
                accept="*.csv"
                { disabled }
                multiple=false
                style="display: none;"
                { onchange }
            />

            <FileUpload
                drag_over={ *drop.over }
            >
                <FileUploadSelect>
                    <Card>
                        <CardTitle>
                            <i class={ Icon::Upload.as_classes() } />
                            { " " }
                            { label_title }
                        </CardTitle>
                        <CardBody>
                            <p style="color: grey;">
                                { label_detail }
                            </p>
                        </CardBody>
                        { footer }
                    </Card>
                </FileUploadSelect>
            </FileUpload>
        </div>
    }
}

trait GenericFileState
where
    Self: 'static + GenericCassetteTaskHandle<FetchState<DataTable>>,
{
    fn set_file_iter(
        &self,
        r#type: DataTableSourceType,
        file: Option<::web_sys::File>,
        length: u32,
    ) {
        match length {
            0 => self.set(FetchState::Pending),
            1 => match file {
                Some(file) => self.set_file_unchecked(r#type, file),
                None => self.set(FetchState::Pending),
            },
            2.. => self.set(FetchState::Error("Cannot get multiple items".into())),
        }
    }

    fn set_file_list(&self, r#type: DataTableSourceType, files: FileList) {
        let length = files.length();
        let file = files.item(0);
        self.set_file_iter(r#type, file, length)
    }

    fn set_file_unchecked(&self, r#type: DataTableSourceType, file: ::web_sys::File) {
        self.set(FetchState::Fetching);

        let name = file.name();
        let size = file.size() as usize;
        let mut stream = ReadableStream::from_raw(file.stream()).into_async_read();

        let state = self.clone();
        spawn_local(async move {
            let mut data = Vec::with_capacity(size);
            state.set(match stream.read_to_end(&mut data).await {
                Ok(_) => match r#type.parse_bytes(data) {
                    Ok(data) => FetchState::Completed(Rc::new(DataTable {
                        name,
                        data: Rc::new(data),
                        log: DataTableLog::default(),
                    })),
                    Err(error) => FetchState::Error(format!("Failed to parse file data: {error}")),
                },
                Err(error) => FetchState::Error(format!("Failed to fetch file data: {error}")),
            })
        })
    }

    fn set_files(&self, r#type: DataTableSourceType, mut files: Vec<::web_sys::File>) {
        let length = files.len() as u32;
        let file = files.pop();
        self.set_file_iter(r#type, file, length)
    }
}

impl GenericFileState for CassetteTaskHandle<FetchState<DataTable>> {}
