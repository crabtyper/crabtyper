use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::use_store;
use gloo::timers::callback::Interval;
use gloo::net::http::Request;

use crate::components::result_view::ResultView;
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

async fn get_random_snippet() -> Result<Snippet, gloo::net::Error> {
    let url = if let Some(url) = option_env!("API_URL") {
        url
    } else {
        "https://crabtyper-api.azurewebsites.net/api/snippets/random"
    };

    Request::get(url).send().await?.json().await
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
                    Status::Loading => {
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Ok(snippet) = get_random_snippet().await {
                                dispatch.apply(Action::NewSnippet(snippet));
                            } else {
                                gloo::console::error!("Error: could not fetch snippets!");
                            }
                        });

                    },
                    Status::Ready => (),
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
            <ResultView />
        } else {
            <Vim />
        }
    }
}
