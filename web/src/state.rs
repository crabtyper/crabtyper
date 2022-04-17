use crate::{
    components::game::Snippet,
    constant::Status,
    utils::calculate::{calculate_accuracy, calculate_progress, calculate_wpm},
};

use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq)]
pub struct Code {
    pub lines: usize,
    pub cursor: Option<char>,
    pub remaining: String,
    pub correct: String,
    pub wrong: String,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Stats {
    pub progress: u8,
    pub mistakes: u8,
    pub wpm: u8,
    pub accuracy: u8,
    pub time: usize,
    pub combos: u8,
}

#[derive(Default, Clone, PartialEq, Store)]
pub struct GameState {
    pub code: Code,
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

impl Reducer<GameState> for Action {
    fn apply(&self, state: &mut GameState) {
        match self {
            Action::NewSnippet(snippet) => {
                *state = GameState::reset();

                let mut chars = snippet.code.chars();

                state.code.cursor = chars.next();
                state.code.remaining = chars.as_str().to_string();
                state.code.lines = snippet.code.lines().count() - 1;

                state.language = snippet.language.clone();
            }

            Action::Tick => {
                state.stats.time += 1;
                state.stats.wpm = calculate_wpm(state.stats.time, &state.code.correct);
            }

            Action::BackSpace => {
                let mut text = &mut state.code;

                if !text.wrong.is_empty() {
                    if let Some(cursor) = text.cursor {
                        if cursor == '❚' {
                            text.remaining = format!("{}{}", ' ', text.remaining);
                        } else {
                            text.remaining = format!("{}{}", cursor, text.remaining);
                        }
                    }

                    text.cursor = text.wrong.pop();
                    if let Some(c) = text.cursor {
                        if c == '❚' {
                            text.cursor = Some(' ')
                        }
                    }

                    while text.cursor == Some('\t') {
                        if let Some(cursor) = text.cursor {
                            text.remaining = format!("{}{}", cursor, text.remaining);
                        }
                        text.cursor = text.wrong.pop();
                    }
                }
            }

            // TODO: clean this up
            Action::KeyPress(key) => {
                if state.status != Status::Passed {
                    state.status = Status::Playing
                };

                let mut text = &mut state.code;
                let mut stats = &mut state.stats;
                let mut chars = text.remaining.chars();

                if let Some(cursor) = text.cursor {
                    if text.wrong.is_empty() && cursor == *key {
                        text.correct.push(*key);

                        text.cursor = chars.next();

                        if text.remaining.is_empty() {
                            stats.accuracy =
                                calculate_accuracy(&text.correct, &text.remaining, stats.mistakes);
                            state.status = Status::Passed;
                        }

                        while text.cursor == Some('\t') {
                            text.correct.push('\t');
                            text.cursor = chars.next();
                        }
                    } else if text.wrong.len() < 10 {
                        stats.mistakes += 1;

                        if let Some(cursor) = text.cursor {
                            if cursor == ' ' {
                                text.wrong.push('❚');
                            } else {
                                text.wrong.push(cursor);
                            }
                        }

                        text.cursor = chars.next();

                        while text.cursor == Some('\t') {
                            text.wrong.push('\t');
                            text.cursor = chars.next();
                        }
                    }
                }

                text.remaining = chars.as_str().to_string();
                stats.progress = calculate_progress(&text.correct, &text.remaining);
            }

            Action::Reset => {
                *state = GameState::reset();
            }
        }
    }
}

impl GameState {
    pub fn reset() -> GameState {
        GameState::default()
    }
}
