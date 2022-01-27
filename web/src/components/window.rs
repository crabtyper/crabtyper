use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::linenumber::LineNumber;
#[derive(Properties, PartialEq)]
pub struct WindowProps {
    pub on_key_press: Callback<KeyboardEvent>,
    pub current_char: String,
    pub typed_text: String,
    pub remaining_text: String,
    pub lines: usize,
}

#[function_component(Window)]
pub fn window(props: &WindowProps) -> Html {
    let input_ref = use_node_ref();

    let onclick = {
        let input_ref = input_ref.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            input.focus().unwrap();
        })
    };

    html! {
        <div>
            <div class="flex flex-row px-6 pt-6 gap-2">
                <LineNumber lines={props.lines}/>
                <pre {onclick} class="relative display-inline w-full break-all" style="tab-size: 4;">
                    <span class="text-white">
                        {"// The code is from Simple FileSharing Service and is licensed under the MIT license."}
                    </span>
                    <br/>
                    <span class="text-blue break-all">{&props.typed_text}</span>
                    <span class="bg-white-light text-black-light">{&props.current_char.to_string()}</span>
                    <span class="text-white">{&props.remaining_text}</span>
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
