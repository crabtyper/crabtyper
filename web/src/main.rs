pub mod components;

mod app;

use app::App;

fn main() {
    yew::start_app::<App>();
}
