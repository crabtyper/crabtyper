use yew::prelude::*;

use crate::components::header::Header;
use crate::components::vim::Vim;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container mx-auto mt-6">
            <div class="flex flex-col gap-16">
                <Header />
                <Vim />
            </div>
        </main>
    }
}
