use crate::{
    components::linenumber::LineNumber,
    external,
    state::{Action, GameState},
};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::{use_selector, Dispatch};

#[function_component]
pub fn Window() -> Html {
    let dispatch = Dispatch::<GameState>::new();
    let code = use_selector(|state: &GameState| state.code.clone());
    let language = use_selector(|state: &GameState| state.language.clone());

    let higlighted = use_state(|| false);

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

    use_effect(move || {
        gloo::console::log!("rendering...");
        || ()
    });

    use_effect_with_deps(
        {
            let remaining = code.remaining.clone();
            move |_| {
                gloo::console::log!("remaining changed");
                if !remaining.is_empty() && !*higlighted {
                    gloo::console::log!("highlighting...");
                    external::Hljs::highlight_all();
                    higlighted.set(true);
                }
                || ()
            }
        },
        code.remaining.clone(),
    );

    let onclick = {
        let input_ref = input_ref.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            input.focus().unwrap();
        })
    };

    let onkeydown = {
        let wrong = code.wrong.clone();

        Callback::from(move |e: KeyboardEvent| {
            let mut key: Option<char> = None;
            let key_string: String = e.key();

            if key_string == "Enter" {
                key = Some('\n');
            } else if key_string == "Tab" {
                e.prevent_default();
                key = Some('\t');
            } else if key_string == "Backspace" && !wrong.is_empty() {
                dispatch.apply(Action::BackSpace)
            } else if key_string.len() == 1 {
                key = key_string.chars().next();
            }

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

    let correct_classes = classes!(
        // "hljs",
        "text-green",
        format!("language-{}", language.to_lowercase())
    );

    html! {
        <div>
            <div class="flex flex-row px-6 pt-6 gap-2">
                <LineNumber lines={code.lines}/>
                <pre {onclick} class="relative display-inline w-full break-all" style="tab-size: 4;">
                    <code class={correct_classes}>{html_escape::encode_safe(&code.correct).to_string()}</code>
                    <code class="text-red">{wrong}</code>
                    <code class="bg-white-light text-black-light">{cursor}</code>
                    <code class="hljs language-rust">{&code.remaining}</code>
                    <input
                        ref={input_ref}
                        {onkeydown}
                        autocomplete="off"
                        type="text"
                        style="position: absolute; width: 1px; left: -10000px;"
                    />
                </pre>
            </div>
        </div>
    }
}
