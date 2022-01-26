use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use gloo::console::debug;
use gloo::timers::callback::Interval;
use yew::prelude::*;

use crate::components::statusline::Statusline;
use crate::components::window::Window;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Insert => write!(f, "INSERT"),
        }
    }
}

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

    let typed_text = use_mut_ref(|| "".to_string());

    let has_started = use_state(|| false);
    let mode = use_state(|| Mode::Normal);
    let sec_past = use_state(|| 0_u32);

    let timer: Rc<RefCell<Option<Interval>>> = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let has_started = has_started.clone();
            let mode = mode.clone();
            let sec_past = sec_past.clone();

            move |_| {
                if *has_started {
                    mode.set(Mode::Insert);
                    let mut sec = *sec_past;
                    *timer.borrow_mut() = Some(Interval::new(1000, move || {
                        sec += 1;
                        sec_past.set(sec);
                    }));
                } else {
                    *timer.borrow_mut() = None;
                    mode.set(Mode::Normal);
                    sec_past.set(0);
                }
                || ()
            }
        },
        has_started.clone(),
    );

    fn format_timer(time: &u32) -> String {
        let time = *time;
        let minutes: f64 = js_sys::Math::floor(f64::from(time) / 60.0);
        let seconds: f64 = f64::from(time) - minutes * 60.0;

        format!("{minutes:}:{seconds:0>2}")
    }

    html! {
        <div class="w-full bg-black-light h-[36rem] shadow-2xl text-lg">
            <div class="flex flex-col justify-between h-full">
                <Window {has_started} {snippet} {typed_text}/>
                <Statusline mode={format!("{}", *mode)} timer={format_timer(&*sec_past)} lang={"Rust"} wpm={120} progress={"TOP"}/>
            </div>
        </div>
    }
}
