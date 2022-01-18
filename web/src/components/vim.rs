use yew::prelude::*;

#[function_component(Vim)]
pub fn vim() -> Html {
    html! {
        <>
            <div class="w-full bg-black-light">
                <p>{0}</p>
                <p>{0}</p>
                <p>{0}</p>
                <p>{0}</p>
                <p>{0}</p>
            </div>
        </>
    }
}
