use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={crate::route::switch} />
        </BrowserRouter>
    }
}
