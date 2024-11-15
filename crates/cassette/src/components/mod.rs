mod actor;
mod file_upload;
mod load;
mod table;
mod text;
mod text_input;
mod variable;

use cassette_core::{
    cassette::{CassetteContext, CassetteState},
    components::ComponentRendererExt,
    task::{CassetteTask, TaskRenderer, TaskResult},
};

pub struct RootCassetteTask<'a>(pub(crate) &'a CassetteTask);

impl TaskRenderer for RootCassetteTask<'_> {
    fn render(&self, state: &mut CassetteState) -> TaskResult<()> {
        let Self { 0: task } = self;

        let CassetteTask {
            name,
            kind,
            metadata: _,
            spec,
        } = task;

        let ctx = CassetteContext::new(state, task);

        match kind.as_str() {
            "Actor" => self::actor::State::render_with(ctx, spec),
            "FileUpload" => self::file_upload::State::render_with(ctx, spec),
            #[cfg(feature = "kubernetes-list")]
            "KubernetesList" => ::cassette_plugin_kubernetes_list::State::render_with(ctx, spec),
            "Load" => self::load::State::render_with(ctx, spec),
            #[cfg(feature = "openai-chat")]
            "OpenAIChat" => ::cassette_plugin_openai_chat::State::render_with(ctx, spec),
            "Table" => self::table::State::render_with(ctx, spec),
            "Text" => self::text::State::render_with(ctx, spec),
            "TextInput" => self::text_input::State::render_with(ctx, spec),
            "Variable" => self::variable::render(ctx, spec),
            "WebcamAudio" => ::cassette_plugin_webcam_audio::State::render_with(ctx, spec),
            _ => Err(format!("Unknown type: {name:?} as {kind}")),
        }
    }
}
