mod app;
mod components;
mod fetch;
mod models;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
