use yew::prelude::*;

mod app;
mod session;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
