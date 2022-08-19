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
    let correct = use_selector(|state: &GameState| state.code.correct.clone());
    let wrong = use_selector(|state: &GameState| state.code.wrong.clone());

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

    let current_line = {
        let mut w = 0;
        let mut c = 0;

        if wrong.lines().count() > 1 {
            w = wrong.lines().count() - 1;
        }

        if correct.lines().count() > 0 {
            c = correct.lines().count();
        }

        let total = w + c;

        if total == 0 {
            1
        } else {
            total
        }
    };

    html! {
        <div class="flex flex-row gap-2">
            <LineNumber lines={*lines} current_line={current_line} />
            <Buffer {input_ref} />
        </div>
    }
}
