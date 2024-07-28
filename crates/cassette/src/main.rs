mod app;
mod build_info;
mod components;
mod history;
mod hooks;
mod pages;
mod panic_hook;
mod route;
mod tracer;

use tracing::Level;

const LOG_LEVEL: Level = if self::build_info::CI_PLATFORM.is_some() {
    Level::TRACE
} else if cfg!(debug_assertions) {
    Level::DEBUG
} else {
    Level::INFO
};

fn main() {
    crate::tracer::init(LOG_LEVEL);
    crate::panic_hook::init();
    ::yew::Renderer::<crate::app::App>::new().render();
}
