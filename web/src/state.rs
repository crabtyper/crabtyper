use std::rc::Rc;

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
    pub combo: u8,
    pub max_combo: u8,
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
    fn apply(&self, mut gamestate: Rc<GameState>) -> Rc<GameState> {
        let mut state = Rc::make_mut(&mut gamestate);

        match self {
            Action::NewSnippet(snippet) => {
                let mut new_state = GameState::reset();

                let mut chars = snippet.code.chars();

                new_state.code.cursor = chars.next();
                new_state.code.remaining = chars.as_str().to_string();
                new_state.code.lines = snippet.code.lines().count() - 1;

                new_state.language = snippet.language.clone();

                gloo::console::log!("state -> changing snippet");

                Rc::new(new_state)
            }

            Action::Tick => {
                state.stats.time += 1;
                state.stats.wpm = calculate_wpm(state.stats.time, &state.code.correct);

                gamestate
            }

            Action::BackSpace => {
                let mut code = &mut state.code;

                if !code.wrong.is_empty() {
                    if let Some(cursor) = code.cursor {
                        if cursor == '❚' {
                            code.remaining = format!("{}{}", ' ', code.remaining);
                        } else {
                            code.remaining = format!("{}{}", cursor, code.remaining);
                        }
                    }

                    code.cursor = code.wrong.pop();
                    if let Some(c) = code.cursor {
                        if c == '❚' {
                            code.cursor = Some(' ')
                        }
                    }

                    while code.cursor == Some('\t') {
                        if let Some(cursor) = code.cursor {
                            code.remaining = format!("{}{}", cursor, code.remaining);
                        }
                        code.cursor = code.wrong.pop();
                    }
                }

                gamestate
            }

            Action::KeyPress(key) => {
                // change status to playing
                if state.status != Status::Passed {
                    state.status = Status::Playing
                };

                let mut code = &mut state.code;
                let mut stats = &mut state.stats;
                let mut chars = code.remaining.chars();

                if let Some(cursor) = code.cursor {
                    // correct key press
                    if code.wrong.is_empty() && cursor == *key {
                        // update combo stats
                        stats.combo += 1;
                        if stats.combo > stats.max_combo {
                            stats.max_combo = stats.combo;
                        }
                        // update the cursor to next key
                        code.correct.push(*key);
                        code.cursor = chars.next();

                        // skip tab characters
                        while code.cursor == Some('\t') {
                            code.correct.push('\t');
                            code.cursor = chars.next();
                        }

                        // end of code snippet
                        if code.remaining.is_empty() {
                            stats.accuracy =
                                calculate_accuracy(&code.correct, &code.remaining, stats.mistakes);
                            state.status = Status::Passed;
                        }
                    // wrong key press
                    } else if code.wrong.len() < 10 {
                        // update stats
                        stats.combo = 0;
                        stats.mistakes += 1;

                        // make space character visible
                        if let Some(cursor) = code.cursor {
                            if cursor == ' ' {
                                code.wrong.push('❚');
                            } else {
                                code.wrong.push(cursor);
                            }
                        }

                        code.cursor = chars.next();

                        // skip tab charcters
                        while code.cursor == Some('\t') {
                            code.wrong.push('\t');
                            code.cursor = chars.next();
                        }
                    }
                }

                code.remaining = chars.as_str().to_string();
                stats.progress = calculate_progress(&code.correct, &code.remaining);

                gloo::console::debug!("state -> keypress");

                gamestate
            }

            Action::Reset => {
                *state = GameState::reset();

                gamestate
            }
        }
    }
}

impl GameState {
    pub fn reset() -> GameState {
        GameState::default()
    }
}
