use reqwasm::http::Request;
use yew::prelude::*;

use crate::components::result::Result;
use crate::components::vim::Vim;
use crate::constant::Status;

use crate::context::gamestate_ctx::{GameStateContext, Action};

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
    let state = use_context::<GameStateContext>().unwrap();

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

    let on_key_press = {
        let state = state.clone();

        Callback::from(move |e: KeyboardEvent| {
            let mut key: Option<char> = None;
            let key_string: String = e.key();

            if key_string == "Enter" {
                key = Some('\n');
            } else if key_string == "Tab" {
                e.prevent_default();
                key = Some('\t');
            } else if key_string == "Backspace" && !state.text.wrong.is_empty() {
                state.dispatch(Action::BackSpace)
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
                }
            }
        })
    };

    let on_reset = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::Reset);
        })
    };

    html! {
        <>
            if state.status == Status::Passed {
                <Result {on_reset} />
            } else {
                <Vim {on_key_press} />
            }
        </>
    }
}
