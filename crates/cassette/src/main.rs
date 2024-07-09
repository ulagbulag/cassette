mod app;
mod components;
mod pages;
mod route;

fn main() {
    ::yew::Renderer::<crate::app::App>::new().render();
}
