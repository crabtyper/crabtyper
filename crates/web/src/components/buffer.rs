use crate::{
    components::cursor::{Cursor, CursorStyle, Position},
    constant::Mode,
    external,
    state::{Action, GameState},
};

use wasm_bindgen::JsValue;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::{use_selector, Dispatch};

#[derive(Properties, PartialEq)]
pub struct BufferProps {
    pub input_ref: NodeRef,
}

#[function_component]
pub fn Buffer(props: &BufferProps) -> Html {
    let dispatch = Dispatch::<GameState>::new();

    let code = use_selector(|state: &GameState| state.code.clone());
    let language = use_selector(|state: &GameState| state.language.clone());

    let correct_ref = use_node_ref();
    let remaining_ref = use_node_ref();

    let cursor_position = use_state(Position::default);

    use_effect_with_deps(
        {
            let correct = code.correct.clone();
            let correct_ref = correct_ref.clone();

            move |_| {
                let correct_code = correct_ref.cast::<HtmlElement>().unwrap();
                if !correct.is_empty() {
                    let options = external::HighlightOptions {
                        language: "rust".to_string(),
                        ignore_illegals: true,
                    };
                    let highlighted: external::HighlightResult = external::Hljs::highlight(
                        &correct,
                        &JsValue::from_serde(&options).unwrap(),
                    );

                    correct_code.set_inner_html(&highlighted.value());
                } else {
                    correct_code.set_inner_html("");
                }
                || ()
            }
        },
        code.correct.clone(),
    );

    use_effect_with_deps(
        {
            let pos = cursor_position.clone();
            let remaining_ref = remaining_ref.clone();
            move |_| {
                let remaining_element = remaining_ref.cast::<HtmlElement>().unwrap();
                let mut new_pos = *pos;
                new_pos.left = remaining_element.offset_left();
                new_pos.top = remaining_element.offset_top();

                pos.set(new_pos);
                || ()
            }
        },
        code.remaining.clone(),
    );

    let onkeydown = {
        let wrong = code.wrong.clone();
        let input_ref = props.input_ref.clone();

        Callback::from(move |e: KeyboardEvent| {
            let key: Option<char> = match e.key().as_str() {
                "Escape" => {
                    let input = input_ref.cast::<HtmlInputElement>().unwrap();
                    input.blur().unwrap();
                    dispatch.apply(Action::ChangeMode(Mode::NORMAL));
                    None
                }
                "Backspace" if !wrong.is_empty() => {
                    dispatch.apply(Action::BackSpace);
                    None
                }
                "Enter" => Some('\n'),
                "Tab" => {
                    e.prevent_default();
                    Some('\t')
                }
                k if k.len() == 1 => k.chars().next(),
                _ => None,
            };

            if let Some(k) = key {
                if k.is_alphanumeric() || k.is_whitespace() || k.is_ascii_punctuation() {
                    dispatch.apply(Action::KeyPress(k));
                }
            }
        })
    };

    let wrong = &code.wrong.replace('\n', "↵\n");

    let cursor = {
        if let Some(cursor) = code.cursor {
            match cursor {
                '\n' => "↵\n".to_string(),
                _ => cursor.to_string(),
            }
        } else {
            "".to_string()
        }
    };

    let hljs_classes = classes!(
        "hljs",
        "text-white",
        format!("language-{}", language.to_lowercase())
    );

    html! {
        <pre class="relative display-inline w-full break-all" style="tab-size: 4;">
            <Cursor position={*cursor_position} smooth={true} style={CursorStyle::Line} />
            <code ref={correct_ref} class={hljs_classes}>{code.correct.clone()}</code>
            <code class="bg-red">{wrong}</code>
            <code ref={remaining_ref} class="text-white">{format!("{}{}", &cursor, &code.remaining)}</code>
            <input
                ref={&props.input_ref}
                id="hidden-input"
                {onkeydown}
                autocomplete="off"
                type="text"
                style="position: absolute; width: 1px; left: -10000px;"
            />
        </pre>
    }
}
