use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LineNumberProps {
    pub lines: usize,
}

#[function_component(LineNumber)]
pub fn line_number(props: &LineNumberProps) -> Html {
    let items = 1..=props.lines + 1;

    html! {
        <div class="text-white w-8">
            {
                items.into_iter().map(|line| {
                    html!{<p key={line}>{line}</p>}
                }).collect::<Html>()
            }
        </div>
    }
}
