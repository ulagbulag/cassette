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
            "Text" => self::text::render(state, spec),
            _ => Err(format!("Unknown type: {name:?} as {kind}")),
        }
    }
}
