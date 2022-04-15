mod components;
mod constant;
mod context;
mod utils;

use crate::components::app::App;

fn main() {
    yew::start_app::<App>();
}
