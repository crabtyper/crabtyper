use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::game::Game;
use crate::components::header::Header;
use crate::components::pagelayout::PageLayout;

#[function_component]
pub fn App() -> Html {
    html! {
        <PageLayout>
            <Header />
            <main>
                <Game/>
            </main>
            <Footer />
        </PageLayout>
    }
}
