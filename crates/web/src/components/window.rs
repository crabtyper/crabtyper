use crate::{
    components::{buffer::Buffer, linenumber::LineNumber},
    constant::Mode,
    state::{Action, GameState},
};

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_event_with_window;
use yewdux::prelude::{use_selector, Dispatch};

#[function_component]
pub fn Window() -> Html {
    let dispatch = Dispatch::<GameState>::new();

    let lines = use_selector(|state: &GameState| state.code.lines);
    let mode = use_selector(|state: &GameState| state.mode);

    let input_ref = use_node_ref();

    use_event_with_window("keyup", {
        let input_ref = input_ref.clone();
        move |e: KeyboardEvent| {
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
        }
    });

    html! {
        <div class="flex flex-row px-6 pt-6 gap-2">
            <LineNumber lines={*lines}/>
            <Buffer {input_ref} />
        </div>
    }
}
