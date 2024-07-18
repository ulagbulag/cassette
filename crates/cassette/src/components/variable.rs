use cassette_core::{
    cassette::CassetteContext,
    task::{TaskResult, TaskSpec, TaskState},
};

pub fn render(ctx: CassetteContext, spec: &TaskSpec) -> TaskResult<()> {
    Ok(TaskState::Skip {
        state: ctx.set_task_state(spec.clone()),
    })
}
