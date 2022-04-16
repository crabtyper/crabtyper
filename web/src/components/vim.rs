use yew::prelude::*;

use crate::components::statusline::Statusline;
use crate::components::window::Window;

#[function_component]
pub fn Vim() -> Html {
    html! {
        <div class="w-full bg-black-light h-[36rem] shadow-2xl text-lg">
            <div class="flex flex-col justify-between h-full">
                <Window/>
                <Statusline />
            </div>
        </div>
    }
}
