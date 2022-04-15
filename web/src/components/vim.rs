use yew::prelude::*;

use crate::components::statusline::Statusline;
use crate::components::window::Window;

#[derive(Properties, PartialEq)]
pub struct VimProps {
    pub on_key_press: Callback<KeyboardEvent>,
}

#[function_component(Vim)]
pub fn vim(props: &VimProps) -> Html {
    html! {
        <div class="w-full bg-black-light h-[36rem] shadow-2xl text-lg">
            <div class="flex flex-col justify-between h-full">
                <Window
                    on_key_press={props.on_key_press.clone()}
                />
                <Statusline />
            </div>
        </div>
    }
}
