mod components;
mod constant;
mod state;
mod utils;

use crate::components::app::App;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    yew::Renderer::<App>::new().render();
}
