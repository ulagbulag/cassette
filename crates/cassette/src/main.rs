mod app;
mod components;
mod hooks;
mod pages;
mod route;

fn main() {
    ::yew::Renderer::<crate::app::App>::new().render();
}
