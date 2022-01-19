use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageLayoutProps {
    pub children: Children,
}

#[function_component(PageLayout)]
pub fn page_layout(props: &PageLayoutProps) -> Html {
    html! {
        <div class="container mx-auto flex flex-col justify-between h-full">
            { for props.children.iter() }
        </div>
    }
}
