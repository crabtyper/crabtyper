use gloo::console::debug;
use yew::prelude::*;

use crate::components::statusline::Statusline;
use crate::components::window::Window;

#[function_component(Vim)]
pub fn vim() -> Html {
    let snippet = "impl Default for FileFlags {
\tfn default() -> Self {
\tSelf {
\t\tpublic: true,
\t\tprotected: false,
\t\tno_preview: false,
\t\t}
\t}
}";

    let has_started = use_state(|| false);

    use_effect_with_deps(
        {
            let has_started = has_started.clone();
            move |_| {
                if *has_started {
                    debug!("started!");
                } else {
                    debug!("finished!");
                }
                || drop(has_started)
            }
        },
        has_started.clone(),
    );

    fn format_timer(time: i32) -> String {
        let minutes: f64 = js_sys::Math::floor(f64::from(time) / 60.0);
        let seconds: f64 = f64::from(time) - minutes * 60.0;

        format!("{minutes:}:{seconds:0>2}")
    }

    html! {
        <div class="w-full bg-black-light h-[36rem] shadow-2xl text-lg">
            <div class="flex flex-col justify-between h-full">
                <Window {has_started} {snippet}/>
                <Statusline mode={"NORMAL"} timer={format_timer(0)} lang={"Rust"} wpm={120} progress={"TOP"}/>
            </div>
        </div>
    }
}
