use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};

use gloo::timers::callback::Interval;
use yew::prelude::*;

use crate::components::statusline::Statusline;
use crate::components::window::Window;
use crate::constant::Status;
use crate::state::{Action, State};

#[function_component(Vim)]
pub fn vim() -> Html {
    let state = use_reducer(State::reset);

    let cursor = use_state(|| "".to_string());

    let sec_past = use_state(|| 0_u32);
    let timer: Rc<RefCell<Option<Interval>>> = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let cursor = cursor.clone();
            let state = state.clone();

            move |_| {
                let current_char = state.text.chars().nth(state.index).unwrap();
                cursor.set(current_char.to_string());
                || ()
            }
        },
        state.clone(),
    );

    use_effect_with_deps(
        {
            let sec_past = sec_past.clone();
            let state = state.clone();

            move |_| {
                if state.status == Status::Playing {
                    let mut sec = *sec_past;
                    *timer.borrow_mut() = Some(Interval::new(1000, move || {
                        sec += 1;
                        sec_past.set(sec);
                    }));
                } else {
                    *timer.borrow_mut() = None;
                    sec_past.set(0);
                    state.dispatch(Action::Reset);
                }
                || ()
            }
        },
        state.status,
    );

    let typed_text = { state.text[..state.index].to_string() };

    let remaining_text = { state.text[state.index + 1..].to_string() };

    let progress = {
        let progress = (((state.index + 1) as f64 / state.text.len() as f64) * 100.0).floor();
        if progress == 0.0 {
            "TOP".to_string()
        } else {
            format!("{progress}%")
        }
    };

    let lines = { state.text.split('\n').count() - 1 };

    let time = {
        let time = *sec_past;
        let minutes: f64 = (f64::from(time) / 60.0).floor();
        let seconds: f64 = f64::from(time) - minutes * 60.0;

        format!("{minutes:}:{seconds:0>2}")
    };

    let next_char = { state.text.chars().nth(state.index + 1) };

    let mode = {
        if state.status == Status::Playing {
            "INSERT".to_string()
        } else {
            "NORMAL".to_string()
        }
    };

    let on_key_press = {
        let cursor = cursor.clone();

        Callback::from(move |e: KeyboardEvent| {
            let key;
            let key_string = e.key();

            if key_string == "Enter" {
                key = '\n';
            } else if key_string == "Tab" {
                e.prevent_default();
                key = '\t';
            } else {
                key = key_string.chars().next().unwrap();
            }

            state.dispatch(Action::KeyPress(key));

            if let Some(next) = next_char {
                if key.to_string() == *cursor && next == '\n' {
                    cursor.set("↵\n".to_string());
                }
            }
        })
    };

    html! {
        <div class="w-full bg-black-light h-[36rem] shadow-2xl text-lg">
            <div class="flex flex-col justify-between h-full">
                <Window
                    current_char={cursor.deref().clone()}
                    {typed_text}
                    {remaining_text}
                    {lines}
                    {on_key_press}
                />
                <Statusline
                    timer={time}
                    lang={"Rust"}
                    wpm={120}
                    {progress}
                    {mode}
                />
            </div>
        </div>
    }
}
