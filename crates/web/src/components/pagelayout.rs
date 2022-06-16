use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageLayoutProps {
    pub children: Children,
}

#[function_component]
pub fn PageLayout(props: &PageLayoutProps) -> Html {
    html! {
        <section class="container mx-auto flex flex-col justify-between h-full">
            { for props.children.iter() }
        </section>
    }
}
