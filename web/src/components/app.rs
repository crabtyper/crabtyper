use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::game::Game;
use crate::components::header::Header;
use crate::components::pagelayout::PageLayout;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <PageLayout>
            <div class="mt-6">
                <Header />
            </div>
            <main>
                <Game/>
            </main>
            <div class="mb-8">
                <Footer />
            </div>
        </PageLayout>
    }
}
