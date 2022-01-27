use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Status {
    Ready,
    Playing,
    Passed,
}
