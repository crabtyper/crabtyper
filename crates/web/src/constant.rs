use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq)]
pub enum Status {
    Loading,
    Ready,
    Playing,
    Passed,
}

impl Default for Status {
    fn default() -> Self {
        Status::Loading
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
pub enum Mode {
    NORMAL,
    INSERT,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::NORMAL
    }
}
