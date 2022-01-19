use yew::prelude::*;

use crate::components::linenumber::LineNumber;

#[function_component(Window)]
pub fn window() -> Html {
    html! {
        <div class="flex flex-row p-6 gap-2">
            <LineNumber />
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
    }
}
