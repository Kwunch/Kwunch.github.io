#![feature(let_chains)]

pub(crate) mod components;
use components::*;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
