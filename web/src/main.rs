mod components;
mod constant;
mod external;
mod state;
mod utils;

use wasm_bindgen::JsValue;

use crate::components::app::App;

fn main() {
    let options = external::ConfigureOptions {
        css_selector: ".hljs",
        ignore_unescaped_html: false,
    };

    external::Hljs::configure(&JsValue::from_serde(&options).unwrap());

    yew::Renderer::<App>::new().render();
}
