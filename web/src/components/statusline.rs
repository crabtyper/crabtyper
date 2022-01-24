use yew::prelude::*;

#[function_component(Statusline)]
pub fn statusline() -> Html {
    html! {
        <>
            <div class="flex flex-row justify-between w-full bg-gray font-bold">
                <div>
                    <p class="bg-green text-black px-4">{"NORMAL"}</p>
                </div>
                <div class="flex flew-row gap-4">
                    <p>{"Javascript"}</p>
                    <p class="bg-green text-black px-4">{"TOP"}</p>
                </div>
            </div>
        </>
    }
}
