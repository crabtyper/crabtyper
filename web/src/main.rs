mod components;
mod constant;
mod helpers;
mod state;

use crate::components::app::App;

fn main() {
    yew::start_app::<App>();
}
