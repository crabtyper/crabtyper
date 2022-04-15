use std::{cell::RefCell, rc::Rc};

use crate::{
    components::game::Snippet,
    constant::Status,
    utils::calculate::{calculate_accuracy, calculate_progress},
};

use gloo::timers::callback::Interval;
use yew::prelude::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Text {
    pub lines: usize,
    pub cursor: Option<char>,
    pub special_char: Option<String>,
    pub remaining: String,
    pub correct: String,
    pub wrong: String,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Stats {
    pub progress: u8,
    pub mistakes: u8,
    pub wpm: u8,
    pub accuracy: u8,
    pub time: usize,
    pub combos: u8,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct GameState {
    pub text: Text,
    pub stats: Stats,
    pub status: Status,
    pub language: String,
}

pub enum Action {
    NewSnippet(Snippet),
    KeyPress(char),
    BackSpace,
    Tick,
    Reset,
}

impl Reducible for GameState {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::NewSnippet(snippet) => {
                let state = GameState::reset();

                let mut chars = snippet.code.chars();

                GameState {
                    text: Text {
                        cursor: chars.next(),
                        remaining: chars.as_str().to_string(),
                        lines: snippet.code.lines().count() - 1,

                        ..state.text
                    },
                    language: snippet.language,
                    ..state
                }
                .into()
            }

            Action::Tick => {
                let time = self.stats.time + 1;

                let wpm = {
                    let minutes_past = time as f32 / 60.0;
                    let index = self.text.correct.len() as f32;

                    if minutes_past > 0.0 {
                        ((index / 5.0) / minutes_past).floor() as u8
                    } else {
                        0_u8
                    }
                };

                let state = (*self).clone();

                GameState {
                    stats: Stats {
                        time,
                        wpm,
                        ..state.stats
                    },
                    ..state
                }
                .into()
            }

            Action::BackSpace => {
                let mut wrong = self.text.wrong.clone();
                let mut remaining = self.text.remaining.clone();
                let mut cursor = self.text.cursor;
                let mut special_char = self.text.special_char.clone();

                if !wrong.is_empty() {
                    special_char = None;

                    if let Some(next) = cursor {
                        remaining = format!("{}{}", next, remaining);
                    }

                    cursor = wrong.pop();

                    if cursor == Some('\t') {
                        while cursor == Some('\t') {
                            if let Some(next) = cursor {
                                remaining = format!("{}{}", next, remaining);
                            }
                            cursor = wrong.pop();
                        }
                    }
                }

                let state = (*self).clone();

                GameState {
                    text: Text {
                        cursor,
                        special_char,
                        wrong,
                        remaining,
                        ..state.text
                    },
                    ..state
                }
                .into()
            }

            // TODO: clean this up
            Action::KeyPress(key) => {
                let mut status = if self.status != Status::Passed {
                    Status::Playing
                } else {
                    self.status
                };

                let mut cursor = self.text.cursor;
                let mut special_char = self.text.special_char.clone();
                let mut remaining = self.text.remaining.clone();
                let mut correct = self.text.correct.clone();
                let mut wrong = self.text.wrong.clone();

                let mut mistakes = self.stats.mistakes;
                let mut accuracy = self.stats.accuracy;

                let mut chars = remaining.chars();

                if let Some(next) = cursor {
                    if wrong.is_empty() && next == key {
                        correct.push(key);

                        cursor = chars.next();
                        special_char = None;

                        if remaining.is_empty() {
                            accuracy = calculate_accuracy(&correct, &remaining, mistakes);
                            status = Status::Passed;
                        }

                        while cursor == Some('\t') {
                            correct.push('\t');
                            cursor = chars.next();
                        }
                    } else if wrong.len() < 10 {
                        mistakes += 1;

                        if let Some(next) = cursor {
                            wrong.push(next);
                        }

                        cursor = chars.next();
                        special_char = None;

                        while cursor == Some('\t') {
                            wrong.push('\t');
                            cursor = chars.next();
                        }
                    }
                }

                remaining = chars.as_str().to_string();
                let progress = calculate_progress(&correct, &remaining);

                let state = (*self).clone();

                GameState {
                    text: Text {
                        cursor,
                        special_char,
                        remaining,
                        wrong,
                        correct,
                        ..state.text
                    },
                    stats: Stats {
                        mistakes,
                        accuracy,
                        progress,
                        ..state.stats
                    },
                    status,
                    ..state
                }
                .into()
            }

            Action::Reset => GameState::reset().into(),
        }
    }
}

impl GameState {
    pub fn reset() -> GameState {
        GameState {
            ..Default::default()
        }
    }
}

pub type GameStateContext = UseReducerHandle<GameState>;

#[derive(Properties, Debug, PartialEq)]
pub struct GameStateProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(GameStateProvider)]
pub fn gameStateProvider(props: &GameStateProviderProps) -> Html {
    let state = use_reducer(GameState::reset);
    let timer: Rc<RefCell<Option<Interval>>> = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let state = state.clone();

            move |_| {
                if state.status == Status::Playing {
                    *timer.borrow_mut() = Some(Interval::new(1000, move || {
                        state.dispatch(Action::Tick);
                    }));
                } else {
                    *timer.borrow_mut() = None;
                }
                || ()
            }
        },
        state.status,
    );

    html! {
        <ContextProvider<GameStateContext> context={state}>
            {props.children.clone()}
        </ContextProvider<GameStateContext>>
    }
}
