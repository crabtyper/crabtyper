use std::{rc::Rc, str::Chars};

use crate::{
    components::game::Snippet,
    constant::{Mode, Status},
    utils::calculate::{calculate_accuracy, calculate_progress, calculate_wpm},
};

use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Code {
    pub lines: usize,
    pub cursor: Option<char>,
    pub remaining: String,
    pub correct: String,
    pub wrong: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub progress: u8,
    pub mistakes: u8,
    pub wpm: u8,
    pub accuracy: u8,
    pub time: usize,
    pub combo: u8,
    pub max_combo: u8,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
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
                state.code.remaining = chars.collect::<String>();
                state.code.lines = snippet.code.lines().count() - 1;

                state.language = snippet.language.clone();
                state.status = Status::Ready;
            }

            Action::Tick => {
                state.stats.time += 1;
                state.stats.wpm = calculate_wpm(state.stats.time, &state.code.correct);
            }

            Action::ChangeMode(mode) => {
                state.mode = *mode;
            }

            Action::BackSpace => {
                GameState::delete_wrong_char(&mut state.code);
            }

            Action::CtrlBackSpace => {
                let code = &mut state.code;

                // TODO: do this in one step?
                while !code.wrong.is_empty() {
                    GameState::delete_wrong_char(code);
                }
            }

            Action::KeyPress(key) => {
                // change status to playing
                if state.status != Status::Passed {
                    state.status = Status::Playing;
                    state.mode = Mode::INSERT;
                };

                let mut code = &mut state.code;
                let mut stats = &mut state.stats;

                let remaining = code.remaining.clone();
                let mut remaining_chars = remaining.chars();

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
                        code.cursor = remaining_chars.next();

                        GameState::skip_tabs(SkipTabsFor::Correct, code, &mut remaining_chars);

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
                        code.cursor = remaining_chars.next();

                        GameState::skip_tabs(SkipTabsFor::Wrong, code, &mut remaining_chars)
                    }
                }

                code.remaining = remaining_chars.collect::<String>();
                stats.progress = calculate_progress(&code.correct, &code.remaining);
            }

            Action::Reset => {
                *state = GameState::reset();
            }
        }

        gamestate
    }
}

#[derive(PartialEq, Eq)]
pub enum SkipTabsFor {
    Correct,
    Wrong,
    Delete,
}

impl GameState {
    pub fn reset() -> GameState {
        GameState::default()
    }

    pub fn skip_tabs(skip_for: SkipTabsFor, code: &mut Code, remaining_chars: &mut Chars) {
        while code.cursor == Some('\t') {
            match skip_for {
                SkipTabsFor::Correct => code.correct.push('\t'),
                SkipTabsFor::Wrong => code.wrong.push('\t'),
                SkipTabsFor::Delete => {
                    if let Some(cursor) = code.cursor {
                        code.remaining = format!("{}{}", cursor, code.remaining);
                    }
                }
            }
            if skip_for == SkipTabsFor::Delete {
                code.cursor = code.wrong.pop();
            } else {
                code.cursor = remaining_chars.next();
            }
        }
    }

    pub fn delete_wrong_char(code: &mut Code) {
        if !code.wrong.is_empty() {
            // move current cursor to remaining
            if let Some(cursor) = code.cursor {
                code.remaining = format!("{}{}", cursor, code.remaining);
            }

            // set the next cursor
            code.cursor = code.wrong.pop();

            GameState::skip_tabs(
                SkipTabsFor::Delete,
                code,
                &mut code.remaining.clone().chars(),
            );
        }
    }
}
