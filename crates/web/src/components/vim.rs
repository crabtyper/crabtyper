use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::Dispatch;

use crate::components::statusline::Statusline;
use crate::components::window::Window;
use crate::constant::Mode;
use crate::state::{Action, GameState};

#[function_component]
pub fn Vim() -> Html {
    let dispatch = Dispatch::<GameState>::new();

    let onclick = {
        Callback::from(move |_| {
            let input = document()
                .get_element_by_id("hidden-input")
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();

            dispatch.apply(Action::ChangeMode(Mode::INSERT));
            input.focus().unwrap();
        })
    };

    html! {
        <div {onclick} class="w-full bg-black-light h-[36rem] shadow-2xl text-lg">
            <div class="flex flex-col justify-between h-full">
                <Window />
                <Statusline />
            </div>
        </div>
    }
}
