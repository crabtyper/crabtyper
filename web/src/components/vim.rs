use yew::prelude::*;

use crate::components::buffer::Buffer;
use crate::components::statusline::Statusline;

#[function_component(Vim)]
pub fn vim() -> Html {
    html! {
        <div class="w-full bg-black-light h-96 shadow-lg">
            <div class="flex flex-col justify-between h-full">
                <Buffer />
                <Statusline />
            </div>
        </div>
    }
}
