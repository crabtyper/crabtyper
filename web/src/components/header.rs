use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="flex flex-row justify-between font-bold">
            <p>{"CrabTyper"}</p>
            <p>{"Brunkel"}</p>
        </header>
    }
}
