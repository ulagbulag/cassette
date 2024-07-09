use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

#[hook]
pub fn use_gateway() -> String {
    let location = use_location().unwrap();
    let query = location.query::<HashMap<String, String>>().unwrap();
    query
        .get("gateway")
        .cloned()
        .unwrap_or_else(|| "/v1/casette".into())
}
