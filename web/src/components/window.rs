use gloo::console::debug;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

use crate::components::linenumber::LineNumber;

#[function_component(Window)]
pub fn window() -> Html {
    let input_ref = use_node_ref();
    let typed_text_ref = use_node_ref();
    let cursor_ref = use_node_ref();
    let remaining_text_ref = use_node_ref();

    use_effect(|| {
        debug!("Window has rendered...");
        || ()
    });

    let onclick = {
        let input_ref = input_ref.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            input.focus().unwrap();
        })
    };

    fn set_next_char(remaining_text: &HtmlElement, typed_text: &HtmlElement, cursor: &HtmlElement) {
        let current_char = cursor.inner_text();
        let mut remaining = remaining_text.inner_text();
        let next_char = remaining.remove(0);

        if next_char == '\n' {
            cursor.set_inner_text("↵\n");
            remaining_text.set_inner_text(&remaining.to_string());
            typed_text.set_inner_text(&*format!("{}{}", typed_text.inner_text(), current_char));
        } else if next_char == '\t' {
            cursor.set_inner_text(&next_char.to_string());
            typed_text.set_inner_text(&*format!("{}{}", typed_text.inner_text(), current_char));
            remaining_text.set_inner_text(&remaining.to_string());
            set_next_char(remaining_text, typed_text, cursor);
        } else {
            cursor.set_inner_text(&next_char.to_string());
            remaining_text.set_inner_text(&remaining.to_string());
            typed_text.set_inner_text(&*format!("{}{}", typed_text.inner_text(), current_char));
        }
    }

    fn is_key_correct(key: &str, current_char: &str) -> bool {
        key == current_char || (key == "Enter" && current_char == "↵\n")
    }

    let onkeydown = {
        let cursor_ref = cursor_ref.clone();
        let remaining_text_ref = remaining_text_ref.clone();
        let typed_text_ref = typed_text_ref.clone();

        Callback::from(move |e: KeyboardEvent| {
            let key = &e.key()[..];

            let typed_text = typed_text_ref.cast::<HtmlElement>().unwrap();
            let remaining_text = remaining_text_ref.cast::<HtmlElement>().unwrap();
            let cursor = cursor_ref.cast::<HtmlElement>().unwrap();

            let remaining = remaining_text.inner_text();

            if !remaining.is_empty() {
                if is_key_correct(key, &cursor.inner_text()[..]) {
                    set_next_char(&remaining_text, &typed_text, &cursor);
                } else {
                }
            }
        })
    };

    html! {
            <div class="flex flex-row px-6 pt-6 gap-2">
                <LineNumber />
                <pre {onclick} class="relative display-inline w-full" style="tab-size: 4;">
                    <span class="text-white">{"// The code is from Simple FileSharing Service and is licensed under the MIT license."}</span>
                        <br/>
                        <span ref={typed_text_ref} class="text-blue" style="word-break: break-all; background: none; display: inline; padding: 0px;">{""}</span>
                        <span ref={cursor_ref} class="bg-white-light text-black-light">{"i"}</span>
                        <span ref={remaining_text_ref} class="text-white" style="word-break: break-all; background: none; padding: 0px; display: inline;">
    {"mpl Default for FileFlags {
\tfn default() -> Self {
\tSelf {
\t\tpublic: true,
\t\tprotected: false,
\t\tno_preview: false,
\t\t}
\t}
}"}                  </span>
                    <input ref={input_ref} class="text-white" autocomplete="off" type="text" {onkeydown} style="position: absolute; width: 1px; left: -10000px;"/>
                </pre>
            </div>
        }
}
