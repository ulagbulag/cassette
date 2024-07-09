use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/c/:id")]
    Cassette { id: Uuid },
    #[at("/error/404")]
    #[not_found]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <crate::pages::home::Home /> },
        Route::Cassette { id } => html! { <crate::pages::home::Home /> },
        Route::NotFound => html! { <crate::pages::error_404::Error404 /> },
    }
}
