use yew::prelude::*;

use crate::components::statusline::Statusline;
use crate::components::window::Window;
use crate::constant::Status;

#[derive(Properties, PartialEq)]
pub struct VimProps {
    pub on_key_press: Callback<KeyboardEvent>,
    pub status: Status,
    pub current_char: String,
    pub typed_text: String,
    pub remaining_text: String,
    pub progress: String,
    pub time: String,
    pub wpm: u32,
    pub lines: usize,
    pub lang: String,
}

#[function_component(Vim)]
pub fn vim(props: &VimProps) -> Html {
    let mode = {
        if props.status == Status::Playing {
            "INSERT".to_string()
        } else {
            "NORMAL".to_string()
        }
    };

    html! {
        <div class="w-full bg-black-light h-[36rem] shadow-2xl text-lg">
            <div class="flex flex-col justify-between h-full">
                <Window
                    current_char={props.current_char.clone()}
                    typed_text={props.typed_text.clone()}
                    remaining_text={props.remaining_text.clone()}
                    lines={props.lines}
                    on_key_press={props.on_key_press.clone()}
                />
                <Statusline
                    timer={props.time.clone()}
                    lang={props.lang.clone()}
                    wpm={props.wpm}
                    progress={props.progress.clone()}
                    {mode}
                />
            </div>
        </div>
    }
}
