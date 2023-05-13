mod app;
mod components;
mod fetch;
mod models;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
