use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use gloo::timers::callback::Interval;
use reqwasm::http::Request;
use yew::prelude::*;

use crate::components::result::Result;
use crate::components::vim::Vim;
use crate::constant::Status;
use crate::state::{Action, State};

use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Snippet {
    pub id: String,
    pub code: String,
    pub language_id: String,
    pub language: String,
}

#[function_component(Game)]
pub fn game() -> Html {
    let state = use_reducer_eq(State::reset);
    let cursor = use_state(|| "".to_string());
    let sec_past = use_state(|| 0_u32);
    let timer: Rc<RefCell<Option<Interval>>> = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let state = state.clone();
            move |_| {
                if state.status == Status::Ready {
                    wasm_bindgen_futures::spawn_local(async move {
                        let snippet: Snippet = Request::get("http://localhost:5000/api/snippet")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();

                        state.dispatch(Action::NewSnippet(snippet));
                    });
                }
                || ()
            }
        },
        state.status,
    );

    use_effect_with_deps(
        {
            let cursor = cursor.clone();
            let state = state.clone();

            move |_| {
                if state.text.len() > 1 {
                    let current_char = state.text.chars().nth(state.index).unwrap();
                    cursor.set(current_char.to_string());
                }
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
                    sec_past.set(0);
                    let mut sec = *sec_past;
                    *timer.borrow_mut() = Some(Interval::new(1000, move || {
                        sec += 1;
                        sec_past.set(sec);
                    }));
                } else {
                    *timer.borrow_mut() = None;
                }
                || ()
            }
        },
        state.status,
    );

    let typed_text = { state.text[..state.index].to_string() };

    let remaining_text = {
        if state.text.len() > 1 {
            state.text[state.index + 1..].to_string()
        } else {
            "".to_string()
        }
    };

    let progress = {
        let progress = (((state.index + 1) as f64 / state.text.len() as f64) * 100.0).floor();
        if progress == 0.0 || state.index == 0 {
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

    let wpm = {
        let minutes_past = *sec_past as f32 / 60.0;
        if minutes_past > 0.0 {
            ((state.index as f32 / 5.0) / minutes_past).floor() as u32
        } else {
            0_u32
        }
    };

    let next_char = { state.text.chars().nth(state.index + 1) };

    let accuracy = {
        let tabs = state.text.chars().filter(|c| c == &'\t').count();
        let chars = (state.text.len() + tabs) as f32;

        if state.mistakes > 0 {
            (100.0 - ((state.mistakes as f32 / chars) * 100.0)) as u8
        } else {
            100
        }
    };

    let on_key_press = {
        let cursor = cursor.clone();
        let state = state.clone();

        Callback::from(move |e: KeyboardEvent| {
            let mut key: Option<char> = None;
            let key_string: String = e.key();

            if key_string == "Enter" {
                key = Some('\n');
            } else if key_string == "Tab" {
                e.prevent_default();
                key = Some('\t');
            } else if key_string.len() == 1 {
                key = key_string.chars().next();
            }

            if let Some(k) = key {
                if k.is_alphanumeric()
                    || k.is_control()
                    || k.is_whitespace()
                    || k.is_ascii_punctuation()
                {
                    state.dispatch(Action::KeyPress(k));
                    if let Some(next) = next_char {
                        if k.to_string() == *cursor && next == '\n' {
                            cursor.set("↵\n".to_string());
                        }
                    }
                }
            }
        })
    };

    let on_reset = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::Reset);
            sec_past.set(0);
        })
    };

    html! {
        <>
            if state.status == Status::Passed {
                <Result
                    {wpm}
                    {time}
                    {accuracy}
                    mistakes={state.mistakes}
                    {on_reset}
                />
            } else {
                <Vim
                    {on_key_press}
                    status={state.status}
                    current_char={cursor.deref().clone()}
                    {typed_text}
                    {remaining_text}
                    {progress}
                    {time}
                    {wpm}
                    {lines}
                    lang={state.language.clone()}
                />
            }
        </>
    }
}
