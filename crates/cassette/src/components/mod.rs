mod text;

use cassette_core::{
    cassette::CassetteState,
    task::{CassetteTask, TaskRenderer, TaskResult},
};
use yew::prelude::*;

pub struct RootCassetteTask<'a>(pub(crate) &'a CassetteTask);

impl TaskRenderer for RootCassetteTask<'_> {
    fn render(&self, state: &UseStateHandle<CassetteState>) -> TaskResult {
        let Self { 0: task } = self;

        let CassetteTask {
            name,
            kind,
            metadata: _,
            spec,
        } = task;

        match kind.as_str() {
            #[cfg(feature = "kubernetes-list")]
            "KubernetesList" => ::cassette_plugin_kubernetes_list::render(state, spec),
            #[cfg(feature = "openai-chat")]
            "OpenAIChat" => ::cassette_plugin_openai_chat::render(state, spec),
            "Text" => self::text::render(state, spec),
            _ => Err(format!("Unknown type: {name:?} as {kind}")),
        }
    }
}
