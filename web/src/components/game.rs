use std::cell::RefCell;
use std::rc::Rc;

use gloo::timers::callback::Interval;
use reqwasm::http::Request;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::components::result::Result;
use crate::components::vim::Vim;
use crate::constant::Status;
use crate::state::{GameState, Action};

use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Snippet {
    pub id: String,
    pub code: String,
    pub language_id: String,
    pub language: String,
}

#[function_component]
pub fn Game() -> Html {
    let (state, dispatch) = use_store::<GameState>();
    let timer: Rc<RefCell<Option<Interval>>> = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let state = state.clone();

            move |_| {
                match state.status {
                    Status::Ready => {
                        wasm_bindgen_futures::spawn_local(async move {
                            let snippet: Snippet = Request::get("/api/snippets/random")
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();

                            dispatch.apply(Action::NewSnippet(snippet));
                        })
                    }
                    Status::Playing => {
                        *timer.borrow_mut() = Some(Interval::new(1000, move || {
                            dispatch.apply(Action::Tick);
                        }));
                    }
                    Status::Passed => {
                        *timer.borrow_mut() = None;
                    }
                }
                || ()
            }
        },
        state.status,
    );

    html! {
        if state.status == Status::Passed {
            <Result />
        } else {
            <Vim />
        }
    }
}
