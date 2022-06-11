use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Serialize, Deserialize)]
pub struct ConfigureOptions<'a> {
    #[serde(rename(serialize = "cssSelector"))]
    pub css_selector: &'a str,
    #[serde(rename(serialize = "ignoreUnescapedHTML"))]
    pub ignore_unescaped_html: bool,
}

#[wasm_bindgen(module = "https://unpkg.com/@highlightjs/cdn-assets@11.5.0/es/highlight.min.js")]
extern "C" {
    #[wasm_bindgen(js_name = "default")]
    pub type Hljs;

    #[wasm_bindgen(static_method_of = Hljs, js_name = "highlightAll", js_class = "default")]
    pub fn highlight_all();

    #[wasm_bindgen(static_method_of = Hljs, js_class = "default" )]
    pub fn configure(options: &JsValue);

    #[wasm_bindgen(static_method_of = Hljs, js_class = "default", js_name = "debugMode" )]
    pub fn debug_mode();

}
