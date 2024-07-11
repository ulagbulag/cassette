mod app;
mod components;
mod hooks;
mod pages;
mod panic_hook;
mod route;
mod tracer;

use tracing::Level;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: Level = Level::INFO;
#[cfg(debug_assertions)]
const LOG_LEVEL: Level = Level::DEBUG;

fn main() {
    crate::tracer::init(LOG_LEVEL);
    crate::panic_hook::init();
    ::yew::Renderer::<crate::app::App>::new().render();
}
