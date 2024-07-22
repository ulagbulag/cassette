use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::task::{TaskResult, TaskState};

pub fn todo<T>() -> TaskResult<Option<T>> {
    Ok(TaskState::Break {
        body: html! { <Todo /> },
        state: None,
    })
}

#[function_component(Todo)]
pub fn todo() -> Html {
    html! {
        <Alert inline=true title="Unimplemented" r#type={AlertType::Warning}>
            { "Not implemented yet!" }
        </Alert>
    }
}
