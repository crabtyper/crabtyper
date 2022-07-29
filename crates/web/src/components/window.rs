use crate::{
    components::{
        cursor::{Cursor, CursorStyle},
        linenumber::LineNumber,
    },
    constant::Mode,
    external,
    state::{Action, GameState},
};
use wasm_bindgen::JsValue;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_event_with_window;
use yewdux::prelude::{use_selector, Dispatch};

#[function_component]
pub fn Window() -> Html {
    let dispatch = Dispatch::<GameState>::new();
    let code = use_selector(|state: &GameState| state.code.clone());
    let language = use_selector(|state: &GameState| state.language.clone());
    let mode = use_selector(|state: &GameState| state.mode);

    let input_ref = use_node_ref();
    let correct_ref = use_node_ref();

    {
        let input_ref = input_ref.clone();
        let dispatch = dispatch.clone();

        use_event_with_window("keyup", move |e: KeyboardEvent| {
            let key = e.key();

            if *mode == Mode::NORMAL {
                if "i" == key {
                    let input = input_ref.cast::<HtmlInputElement>().unwrap();
                    dispatch.apply(Action::ChangeMode(Mode::INSERT));
                    input.focus().unwrap();
                } else if "n" == key {
                    dispatch.apply(Action::Reset);
                }
            }
        });
    }

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

    let onkeydown = {
        let wrong = code.wrong.clone();
        let input_ref = input_ref.clone();

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
        <div class="flex flex-row px-6 pt-6 gap-2">
            <LineNumber lines={code.lines}/>
            <Cursor style={CursorStyle::Line} current_char={cursor.clone()}/>
            <pre class="relative display-inline w-full break-all" style="tab-size: 4;">
                <code ref={correct_ref} class={hljs_classes} />
                <code class="bg-red">{wrong}</code>
                <code class="text-white">{cursor.clone()}</code>
                <code class="text-white">{&code.remaining}</code>
                <input
                    ref={input_ref}
                    id="hidden-input"
                    {onkeydown}
                    autocomplete="off"
                    type="text"
                    style="position: absolute; width: 1px; left: -10000px;"
                />
            </pre>
        </div>
    }
}
