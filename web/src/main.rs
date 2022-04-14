mod components;
mod constant;
mod context;

use crate::components::app::App;

fn main() {
    yew::start_app::<App>();
}
