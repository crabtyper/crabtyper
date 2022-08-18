use std::cell::RefCell;
use std::rc::Rc;

use gloo::net::http::Request;
use gloo::timers::callback::Interval;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::{use_selector, use_store, Dispatch};

use crate::components::result_view::ResultView;
use crate::components::vim::Vim;
use crate::constant::Status;
use crate::state::{Action, GameState};

use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, Deserialize, Debug)]
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
    let dispatch = Dispatch::<GameState>::new();
    let status = use_selector(|state: &GameState| state.status);

    let timer: Rc<RefCell<Option<Interval>>> = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let status = status.clone();
            move |_| {
                match *status {
                    Status::Loading => {
                        spawn_local(async move {
                            if let Ok(snippet) = get_random_snippet().await {
                                dispatch.apply(Action::NewSnippet(snippet));
                            } else {
                                gloo::console::error!("Error: could not fetch snippets!");
                            }
                        });
                    }
                    Status::Ready => {
                        *timer.borrow_mut() = None;
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
        status.clone(),
    );

    html! {
        if *status == Status::Passed {
            <ResultView />
        } else {
            <Vim />
        }
    }
}
