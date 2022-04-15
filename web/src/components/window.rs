use crate::{components::linenumber::LineNumber, context::gamestate_ctx::GameStateContext};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WindowProps {
    pub on_key_press: Callback<KeyboardEvent>,
}

#[function_component(Window)]
pub fn window(props: &WindowProps) -> Html {
    let state = use_context::<GameStateContext>().unwrap();

    let input_ref = use_node_ref();

    let onclick = {
        let input_ref = input_ref.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            input.focus().unwrap();
        })
    };

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

    let cursor = {
        if let Some(next) = state.text.cursor {
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
                <LineNumber lines={state.text.lines}/>
                <pre {onclick} class="relative display-inline w-full break-all" style="tab-size: 4;">
                    <code class="text-green">{state.text.correct.clone()}</code>
                    <code class="text-red">{state.text.wrong.clone()}</code>
                    <code class="bg-white-light text-black-light">{cursor}</code>
                    <code class="text-white">{state.text.remaining.clone()}</code>
                    <input
                        ref={input_ref}
                        onkeydown={props.on_key_press.clone()}
                        class="text-white"
                        autocomplete="off"
                        type="text"
                        style="position: absolute; width: 1px; left: -10000px;"
                    />
                </pre>
            </div>
        </div>
    }
}
