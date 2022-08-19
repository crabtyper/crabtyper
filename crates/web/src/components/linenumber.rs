use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct LineNumberProps {
    pub lines: usize,
    pub current_line: usize,
}

#[function_component(LineNumber)]
pub fn line_number(props: &LineNumberProps) -> Html {
    let items = 1..=props.lines + 1;

    html! {
        <div class="text-gray-light text-right w-10 mr-4">
            {
                items.into_iter().map(|line| {
                    let mut class = "".to_string();

                    if line == props.current_line {
                        class = "text-white".to_string();
                    }
                    html!{<p {class} key={line}>{line}</p>}
                }).collect::<Html>()
            }
        </div>
    }
}
