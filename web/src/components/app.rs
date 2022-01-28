use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::pagelayout::PageLayout;
use crate::components::vim::Vim;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <PageLayout>
            <div class="mt-6">
                <Header />
            </div>
            <main>
                <Vim/>
            </main>
            <div class="mb-8">
                <Footer />
            </div>
        </PageLayout>
    }
}
