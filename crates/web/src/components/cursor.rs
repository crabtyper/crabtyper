use yew::prelude::*;

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
    pub style: CursorStyle,
    pub smooth: bool,
    pub position: Position,
}

#[function_component]
pub fn Cursor(props: &CursorProps) -> Html {
    let smooth_class = {
        if props.smooth {
            Some(String::from("transition-all duration-75"))
        } else {
            None
        }
    };

    let pos = props.position;
    let style = format!(
        "left: {}px; top: {}px; right: {}px; bottom: {}px",
        pos.left, pos.top, pos.right, pos.bottom
    );

    match props.style {
        CursorStyle::Line => html! {
            <span
                class={classes!("absolute", "bg-white-light", "w-0.5", "h-6", smooth_class)}
                {style}
            />
        },
        CursorStyle::Block => html! {},
    }
}
