use gloo::console::debug;
use gloo::timers::callback::Interval;
use yew::prelude::*;

use crate::components::statusline::Statusline;
use crate::components::window::Window;

#[function_component(Vim)]
pub fn vim() -> Html {
    let timer = use_state(|| 0);
    let snippet = "impl Default for FileFlags {
\tfn default() -> Self {
\tSelf {
\t\tpublic: true,
\t\tprotected: false,
\t\tno_preview: false,
\t\t}
\t}
}";

    //
    // use_effect_with_deps(
    //     {
    //         let timer = timer.clone();
    //         move |_| {
    //             debug!(*timer);
    //             || ()
    //         }
    //     },
    //     vec![*timer],
    // );

    fn start_timer() {
        Interval::new(1000, move || debug!("ok"));
    }

    //
    // // fn stop_timer() {
    // //     let timer_interval = timer_interval.clone();
    // //     timer_interval
    // // }
    // //
    // //
    // fn update_time() {
    //     let timer = timer.clone();
    //     timer.set(*timer + 1);
    // }
    //
    fn format_timer(time: i32) -> String {
        let minutes: f64 = js_sys::Math::floor(f64::from(time) / 60.0);
        let seconds: f64 = f64::from(time) - minutes * 60.0;

        format!("{minutes:}:{seconds:0>2}")
    }

    let onkeydown = {
        let timer = timer.clone();

        Callback::from(move |is_finished: bool| {
            if is_finished {
            } else if *timer == 0 {
                start_timer();
                timer.set(*timer + 1);
            } else {
                timer.set(*timer + 1);
            }
        })
    };

    html! {
        <div class="w-full bg-black-light h-96 shadow-lg">
            <div class="flex flex-col justify-between h-full">
                <Window onkeydown={onkeydown} snippet={snippet}/>
                <Statusline mode={"NORMAL"} timer={format_timer(*timer)} lang={"Rust"} wpm={120} progress={"TOP"}/>
            </div>
        </div>
    }
}
