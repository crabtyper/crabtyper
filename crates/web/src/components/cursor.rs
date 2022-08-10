use yew::prelude::*;
use yewdux::prelude::use_selector;

use crate::{constant::Mode, state::GameState};

#[derive(PartialEq)]
pub enum CursorStyle {
    Line,
    Block,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub struct Position {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Properties, PartialEq)]
pub struct CursorProps {
    pub cursor: String,
    pub style: CursorStyle,
    pub smooth: bool,
    pub position: Position,
}

#[function_component]
pub fn Cursor(props: &CursorProps) -> Html {
    let mode = use_selector(|state: &GameState| state.mode);

    let smooth_class = {
        if props.smooth {
            Some(String::from("transition-all duration-75"))
        } else {
            None
        }
    };

    let pos = props.position;
    let style = format!(
        "height: 24px; left: {}px; top: {}px; right: {}px; bottom: {}px",
        pos.left, pos.top, pos.right, pos.bottom
    );

    let base_classes = String::from("absolute bg-white-light");

    html! {
        if *mode == Mode::NORMAL || props.style == CursorStyle::Block {
            <div
                class={classes!(&base_classes, "text-black-light", "overflow-hidden", "inline-block", "w-fit", "text-center")}
                style={style.clone()}
            >
                {&props.cursor}
            </div>
        } else {
            <span
                class={classes!(base_classes, "w-0.5", smooth_class)}
                {style}
            />
        }

    }
}
