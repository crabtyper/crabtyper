use std::rc::Rc;

use yew::prelude::*;

use crate::constant::Status;

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub text: String,
    pub wrong_text: String,
    pub index: usize,
    pub mistakes: u8,
    pub status: Status,
}

pub enum Action {
    KeyPress(char),
    Reset,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            // TODO: clean this up
            Action::KeyPress(key) => {
                let mut status = if self.status != Status::Passed {
                    Status::Playing
                } else {
                    self.status
                };
                let mut index = self.index;

                let current_char = self.text.chars().nth(index).unwrap();

                if current_char == key {
                    if (index + 1) == self.text.len() {
                        status = Status::Passed;
                    } else {
                        index += 1;
                        let mut next_char = self.text.chars().nth(index).unwrap();

                        while next_char == '\t' && index < self.text.len() {
                            index += 1;
                            next_char = self.text.chars().nth(index).unwrap();
                        }
                    }
                }

                State {
                    text: self.text.clone(),
                    wrong_text: self.wrong_text.clone(),
                    index,
                    mistakes: self.mistakes,
                    status,
                }
                .into()
            }
            Action::Reset => State::reset().into(),
        }
    }
}

impl State {
    pub fn reset() -> State {
        State {
            text: "impl Default for FileFlags {
\tfn default() -> Self {
\tSelf {
\t\tpublic: true,
\t\tprotected: false,
\t\tno_preview: false,
\t\t}
\t}
}"
            .to_string(),
            wrong_text: "".to_string(),
            index: 0,
            mistakes: 0,
            status: Status::Ready,
        }
    }
}
