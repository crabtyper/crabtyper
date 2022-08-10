use std::rc::Rc;

use crate::{
    components::game::Snippet,
    constant::{Mode, Status},
    utils::calculate::{calculate_accuracy, calculate_progress, calculate_wpm},
};

use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Code {
    pub lines: usize,
    pub cursor: Option<char>,
    pub remaining: String,
    pub correct: String,
    pub wrong: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Stats {
    pub progress: u8,
    pub mistakes: u8,
    pub wpm: u8,
    pub accuracy: u8,
    pub time: usize,
    pub combo: u8,
    pub max_combo: u8,
}

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct GameState {
    pub code: Code,
    pub stats: Stats,
    pub status: Status,
    pub mode: Mode,
    pub language: String,
}

pub enum Action {
    NewSnippet(Snippet),
    KeyPress(char),
    BackSpace,
    CtrlBackSpace,
    ChangeMode(Mode),
    Tick,
    Reset,
}

impl Reducer<GameState> for Action {
    fn apply(&self, mut gamestate: Rc<GameState>) -> Rc<GameState> {
        let mut state = Rc::make_mut(&mut gamestate);

        match self {
            Action::NewSnippet(snippet) => {
                *state = GameState::reset();

                let mut chars = snippet.code.chars();

                state.code.cursor = chars.next();
                state.code.remaining = chars.as_str().to_string();
                state.code.lines = snippet.code.lines().count() - 1;

                state.language = snippet.language.clone();
                state.status = Status::Ready;

                gamestate
            }

            Action::Tick => {
                state.stats.time += 1;
                state.stats.wpm = calculate_wpm(state.stats.time, &state.code.correct);

                gamestate
            }

            Action::ChangeMode(mode) => {
                state.mode = *mode;

                gamestate
            }

            Action::BackSpace => {
                let mut code = &mut state.code;

                if !code.wrong.is_empty() {
                    if let Some(cursor) = code.cursor {
                        code.remaining = format!("{}{}", cursor, code.remaining);
                    }

                    code.cursor = code.wrong.pop();

                    while code.cursor == Some('\t') {
                        if let Some(cursor) = code.cursor {
                            code.remaining = format!("{}{}", cursor, code.remaining);
                        }
                        code.cursor = code.wrong.pop();
                    }
                }

                gamestate
            }

            Action::CtrlBackSpace => {
                let mut code = &mut state.code;

                while !code.wrong.is_empty() {
                    if let Some(cursor) = code.cursor {
                        code.remaining = format!("{}{}", cursor, code.remaining);
                    }

                    code.cursor = code.wrong.pop();

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
                    state.status = Status::Playing;
                    state.mode = Mode::INSERT;
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

                        code.wrong.push(cursor);
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
