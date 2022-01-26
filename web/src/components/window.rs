use std::{cell::RefCell, rc::Rc};

use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

use crate::components::linenumber::LineNumber;

#[derive(Properties, PartialEq)]
pub struct WindowProps {
    pub has_started: UseStateHandle<bool>,
    pub typed_text: Rc<RefCell<String>>,
    pub snippet: String,
}

#[function_component(Window)]
pub fn window(props: &WindowProps) -> Html {
    let input_ref = use_node_ref();
    let cursor_ref = use_node_ref();
    let remaining_text_ref = use_node_ref();

    fn set_next_char(
        remaining_text: &HtmlElement,
        typed_text: Rc<RefCell<String>>,
        cursor: &HtmlElement,
    ) {
        let mut current_char = cursor.inner_text();
        let mut remaining = remaining_text.inner_text();
        let next_char = remaining.remove(0);

        if current_char == "↵\n" {
            current_char = String::from("enter");
        };

        let typed_text = Rc::clone(&typed_text);

        typed_text.borrow_mut().push_str(&current_char);

        match next_char {
            '\n' => {
                remaining_text.set_inner_text(&remaining);
                cursor.set_inner_text("↵\n");
            }
            '\t' => {
                remaining_text.set_inner_text(&remaining);
                cursor.set_inner_text(&next_char.to_string());
                set_next_char(remaining_text, typed_text, cursor);
            }
            _ => {
                remaining_text.set_inner_text(&remaining);
                cursor.set_inner_text(&next_char.to_string());
            }
        }
    }

    fn is_key_correct(key: &str, current_char: &str) -> bool {
        key == current_char || (key == "Enter" && current_char == "↵\n")
    }

    fn is_first_key(remaining: &str, snippet: &str) -> bool {
        remaining == &snippet[1..]
    }

    let onclick = {
        let input_ref = input_ref.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            input.focus().unwrap();
        })
    };

    let onkeydown = {
        let has_started = props.has_started.clone();
        let snippet = props.snippet.clone();

        let cursor_ref = cursor_ref.clone();
        let remaining_text_ref = remaining_text_ref.clone();
        let typed_text = Rc::clone(&props.typed_text);

        Callback::from(move |e: KeyboardEvent| {
            let key = &e.key()[..];

            let remaining_text = remaining_text_ref.cast::<HtmlElement>().unwrap();
            let cursor = cursor_ref.cast::<HtmlElement>().unwrap();

            let remaining = remaining_text.inner_text();

            if !remaining.is_empty() {
                if is_first_key(&remaining, &snippet) {
                    has_started.set(true);
                }
                if is_key_correct(key, &cursor.inner_text()) {
                    let typed_text = Rc::clone(&typed_text);
                    set_next_char(&remaining_text, typed_text, &cursor);
                }
            } else if *has_started {
                has_started.set(false);
            }
        })
    };

    html! {
        <div>
            <div class="flex flex-row px-6 pt-6 gap-2">
                <LineNumber lines={props.snippet.lines().count()}/>
                <pre {onclick} class="relative display-inline w-full break-all" style="tab-size: 4;">
                    <span class="text-white">
                        {"// The code is from Simple FileSharing Service and is licensed under the MIT license."}
                    </span>
                    <br/>
                    <span class="text-blue break-all">{&*props.typed_text.borrow()}</span>
                    <span ref={cursor_ref} class="bg-white-light text-black-light">{&props.snippet[..1]}</span>
                    <span ref={remaining_text_ref} class="text-white">{&props.snippet[1..]}</span>
                    <input ref={input_ref} {onkeydown} class="text-white" autocomplete="off" type="text" style="position: absolute; width: 1px; left: -10000px;"/>
                </pre>
            </div>
        </div>
    }
}
