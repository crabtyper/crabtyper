use yew::prelude::*;

use crate::components::statusline::Statusline;

#[function_component(Vim)]
pub fn vim() -> Html {
    html! {
        <div class="w-full bg-black-light h-96 shadow-lg">
            <div class="flex flex-col justify-between h-full">
                <div class="flex flex-row p-6 gap-2">
                    <div class="text-white">
                        <p>{"1"}</p>
                        <p>{"2"}</p>
                        <p>{"3"}</p>
                        <p>{"4"}</p>
                        <p>{"5"}</p>
                        <p>{"6"}</p>
                        <p>{"7"}</p>
                    </div>
                    <div>
                        <p>{"// The code is from Simple FileSharing Service and is licensed under the MIT license."}</p>
                        <p>{"let ctx = PageContext {"}</p>
                        <p>{"\u{00A0} \u{00A0} code: data.hash,"}</p>
                        <p>{"\u{00A0} \u{00A0} url: APP_CONTEXT.url.clone(),"}</p>
                        <p>{"\u{00A0} \u{00A0} webroot: APP_CONTEXT.webroot.clone(),"}</p>
                        <p>{"\u{00A0} \u{00A0} password: data.password,"}</p>
                        <p>{"};"}</p>
                    </div>
                </div>
                <Statusline />
            </div>
        </div>
    }
}
