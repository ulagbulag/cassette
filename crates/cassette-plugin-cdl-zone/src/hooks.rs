use std::rc::Rc;

use cassette_core::{
    cassette::{CassetteContext, CassetteTaskHandle},
    data::table::DataTable,
    net::{
        fetch::{FetchRequestWithoutBody, FetchState, Method},
        gateway::get_gateway,
    },
};

pub fn use_fetch(
    ctx: &mut CassetteContext,
    base_url: Option<String>,
    force: bool,
) -> CassetteTaskHandle<FetchState<Rc<DataTable>>> {
    let handler_name = "chat completions";
    let state = ctx.use_state(handler_name, force, || FetchState::Pending);
    {
        let state = state.clone();
        let base_url = base_url.unwrap_or(get_gateway());
        let request = FetchRequestWithoutBody {
            method: Method::GET,
            name: handler_name,
            url: "/cdl/zone",
            body: None,
        };

        request.try_fetch(&base_url, state)
    }
    state
}
