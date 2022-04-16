mod components;
mod constant;
mod state;
mod utils;

use crate::components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
