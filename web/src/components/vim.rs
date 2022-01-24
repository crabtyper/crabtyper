use std::borrow::BorrowMut;
use std::rc::Rc;

use gloo::console::debug;
use gloo::timers::callback::Interval;
use yew::prelude::*;

use crate::components::statusline::Statusline;
use crate::components::window::Window;

enum TimerAction {
    Increment,
}

struct TimerState {
    timer: i32,
}

impl Default for TimerState {
    fn default() -> Self {
        Self { timer: 0 }
    }
}

impl Reducible for TimerState {
    /// Reducer Action Type
    type Action = TimerAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_ctr = match action {
            TimerAction::Increment => self.timer + 1,
        };

        Self { timer: next_ctr }.into()
    }
}

#[function_component(Vim)]
pub fn vim() -> Html {
    let timer = use_reducer(TimerState::default);
    let has_started = use_state(|| false);
    let snippet = "impl Default for FileFlags {
\tfn default() -> Self {
\tSelf {
\t\tpublic: true,
\t\tprotected: false,
\t\tno_preview: false,
\t\t}
\t}
}";

    let mut interval: Option<Interval> = None;

    let start_timer = {
        let timer = timer.clone();
        move || {
            interval = Some(Interval::new(1_000, move || {
                timer.dispatch(TimerAction::Increment);
            }));
        }
    };

    use_effect_with_deps(
        {
            let has_started = has_started.clone();
            move |_| {
                if *has_started {
                    debug!("started!");
                    start_timer();
                } else {
                    debug!("finished!");
                    interval.unwrap().cancel();
                }
                || drop(has_started)
            }
        },
        has_started.clone(),
    );

    // use_effect(move || {
    //     let timer = timer.clone();
    //     debug!(timer.timer);
    //     let interval = Interval::new(1_000, move || timer.dispatch(TimerAction::Increment));
    //     move || drop(interval)
    // });

    // let on_correct_key = {
    //     let timer = timer.clone();
    //
    //     Callback::from(move |is_finished: bool| {
    //         if is_finished {
    //             debug!("finished!");
    //         }
    //     })
    // };
    //

    fn format_timer(time: i32) -> String {
        let minutes: f64 = js_sys::Math::floor(f64::from(time) / 60.0);
        let seconds: f64 = f64::from(time) - minutes * 60.0;

        format!("{minutes:}:{seconds:0>2}")
    }

    html! {
        <div class="w-full bg-black-light h-96 shadow-lg">
            <div class="flex flex-col justify-between h-full">
                <Window {has_started} {snippet}/>
                <Statusline mode={"NORMAL"} timer={format_timer(timer.timer)} lang={"Rust"} wpm={120} progress={"TOP"}/>
            </div>
        </div>
    }
}
