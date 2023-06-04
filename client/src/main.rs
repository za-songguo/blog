mod app;
mod components;
mod constants;
mod fetch;
mod models;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
