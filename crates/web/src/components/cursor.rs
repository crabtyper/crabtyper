use yew::prelude::*;

#[derive(PartialEq)]
pub enum CursorStyle {
    Line,
    Block,
    Smooth,
}

#[derive(Properties, PartialEq)]
pub struct CursorProps {
    pub style: CursorStyle,
    pub current_char: String,
}

#[function_component]
pub fn Cursor(_props: &CursorProps) -> Html {
    html! {}
}
