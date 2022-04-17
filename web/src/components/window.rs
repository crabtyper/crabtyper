use crate::{
    components::linenumber::LineNumber,
    state::{Action, GameState},
};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::{use_selector, Dispatch};

#[function_component]
pub fn Window() -> Html {
    let dispatch = Dispatch::<GameState>::new();
    let code = use_selector(|state: &GameState| state.code.clone());

    let input_ref = use_node_ref();

    use_effect_with_deps(
        {
            let input_ref = input_ref.clone();
            move |_| {
                let input = input_ref.cast::<HtmlInputElement>().unwrap();
                input.focus().unwrap();
                || ()
            }
        },
        (),
    );

    let onclick = {
        let input_ref = input_ref.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            input.focus().unwrap();
        })
    };

    let onkeydown = {
        let code = code.clone();

        Callback::from(move |e: KeyboardEvent| {
            let mut key: Option<char> = None;
            let key_string: String = e.key();

            if key_string == "Enter" {
                key = Some('\n');
            } else if key_string == "Tab" {
                e.prevent_default();
                key = Some('\t');
            } else if key_string == "Backspace" && !code.wrong.is_empty() {
                dispatch.apply(Action::BackSpace)
            } else if key_string.len() == 1 {
                key = key_string.chars().next();
            }

            if let Some(k) = key {
                if k.is_alphanumeric()
                    || k.is_control()
                    || k.is_whitespace()
                    || k.is_ascii_punctuation()
                {
                    dispatch.apply(Action::KeyPress(k));
                }
            }
        })
    };

    let cursor = {
        if let Some(next) = code.cursor {
            if next == '\n' {
                "↵\n".to_string()
            } else {
                next.to_string()
            }
        } else {
            "".to_string()
        }
    };

    html! {
        <div>
            <div class="flex flex-row px-6 pt-6 gap-2">
                <LineNumber lines={code.lines}/>
                <pre {onclick} class="relative display-inline w-full break-all" style="tab-size: 4;">
                    <code class="text-green">{code.correct.clone()}</code>
                    <code class="text-red">{code.wrong.clone()}</code>
                    <code class="bg-white-light text-black-light">{cursor}</code>
                    <code class="text-white">{code.remaining.clone()}</code>
                    <input
                        ref={input_ref}
                        {onkeydown}
                        class="code-white"
                        autocomplete="off"
                        type="text"
                        style="position: absolute; width: 1px; left: -10000px;"
                    />
                </pre>
            </div>
        </div>
    }
}
