mod text;

use yew::prelude::*;

pub fn route(kind: &str) -> Html {
    match kind {
        "text" => html! { <self::text::Component /> },
        #[cfg(feature = "kubernetes")]
        "kubernetes" => todo!(),
        _ => html! { <crate::pages::error_404::Error404 /> },
    }
}
