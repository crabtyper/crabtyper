use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::use_event_with_window;
use yewdux::prelude::{use_selector, Dispatch};

use crate::state::{Action, GameState};

#[derive(Properties, PartialEq)]
pub struct StatItemProps {
    #[prop_or_default]
    pub children: Children,
    pub value: String,
}

#[function_component]
pub fn StatItem(props: &StatItemProps) -> Html {
    html! {
    <div class="flex gap-3 items-center">
        {for props.children.iter()}
        <span>{&props.value}</span>
    </div>
    }
}

#[function_component]
pub fn ResultView() -> Html {
    let dispatch = Dispatch::<GameState>::new();
    let stats = use_selector(|state: &GameState| state.stats);

    use_event_with_window("keyup", {
        move |event: KeyboardEvent| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
            if event.key() == "n" {
                dispatch.apply(Action::Reset);
            }
        }
    });

    html! {
        <section class="flex flex-col justify-center items-center gap-6">
            <div class="flex flex-col items-center gap-2">
                <h1 class="text-4xl font-bold">{format!("You typed {} WPM!", stats.wpm)}</h1>
                <img src="img/crab.png"  class="w-[300px]"/>
            </div>
            <div class="flex flex-col items-center gap-3">
                <div class="flex gap-7 text-xl font-bold">
                    <StatItem value={stats.time.to_string()}>
                        <svg class="h-6" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="clock" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256,8C119,8,8,119,8,256S119,504,256,504,504,393,504,256,393,8,256,8Zm92.49,313h0l-20,25a16,16,0,0,1-22.49,2.5h0l-67-49.72a40,40,0,0,1-15-31.23V112a16,16,0,0,1,16-16h32a16,16,0,0,1,16,16V256l58,42.5A16,16,0,0,1,348.49,321Z"></path></svg>
                    </StatItem>
                    <StatItem value={format!("{} %", stats.accuracy)}>
                        <svg class="h-6" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="crosshairs" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M500 224h-30.364C455.724 130.325 381.675 56.276 288 42.364V12c0-6.627-5.373-12-12-12h-40c-6.627 0-12 5.373-12 12v30.364C130.325 56.276 56.276 130.325 42.364 224H12c-6.627 0-12 5.373-12 12v40c0 6.627 5.373 12 12 12h30.364C56.276 381.675 130.325 455.724 224 469.636V500c0 6.627 5.373 12 12 12h40c6.627 0 12-5.373 12-12v-30.364C381.675 455.724 455.724 381.675 469.636 288H500c6.627 0 12-5.373 12-12v-40c0-6.627-5.373-12-12-12zM288 404.634V364c0-6.627-5.373-12-12-12h-40c-6.627 0-12 5.373-12 12v40.634C165.826 392.232 119.783 346.243 107.366 288H148c6.627 0 12-5.373 12-12v-40c0-6.627-5.373-12-12-12h-40.634C119.768 165.826 165.757 119.783 224 107.366V148c0 6.627 5.373 12 12 12h40c6.627 0 12-5.373 12-12v-40.634C346.174 119.768 392.217 165.757 404.634 224H364c-6.627 0-12 5.373-12 12v40c0 6.627 5.373 12 12 12h40.634C392.232 346.174 346.243 392.217 288 404.634zM288 256c0 17.673-14.327 32-32 32s-32-14.327-32-32c0-17.673 14.327-32 32-32s32 14.327 32 32z"></path></svg>
                    </StatItem>
                    <StatItem value={stats.max_combo.to_string()}>
                        <svg class="h-6" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="fire" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M323.5 51.25C302.8 70.5 284 90.75 267.4 111.1C240.1 73.62 206.2 35.5 168 0C69.75 91.12 0 210 0 281.6C0 408.9 100.2 512 224 512s224-103.1 224-230.4C448 228.4 396 118.5 323.5 51.25zM304.1 391.9C282.4 407 255.8 416 226.9 416c-72.13 0-130.9-47.73-130.9-125.2c0-38.63 24.24-72.64 72.74-130.8c7 8 98.88 125.4 98.88 125.4l58.63-66.88c4.125 6.75 7.867 13.52 11.24 19.9C364.9 290.6 353.4 357.4 304.1 391.9z"/></svg>
                    </StatItem>
                    <StatItem value={stats.mistakes.to_string()}>
                        <svg class="h-6" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="times-circle" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 8C119 8 8 119 8 256s111 248 248 248 248-111 248-248S393 8 256 8zm121.6 313.1c4.7 4.7 4.7 12.3 0 17L338 377.6c-4.7 4.7-12.3 4.7-17 0L256 312l-65.1 65.6c-4.7 4.7-12.3 4.7-17 0L134.4 338c-4.7-4.7-4.7-12.3 0-17l65.6-65-65.6-65.1c-4.7-4.7-4.7-12.3 0-17l39.6-39.6c4.7-4.7 12.3-4.7 17 0l65 65.7 65.1-65.6c4.7-4.7 12.3-4.7 17 0l39.6 39.6c4.7 4.7 4.7 12.3 0 17L312 256l65.6 65.1z"></path></svg>
                    </StatItem>
                </div>
                <p class="text-white text-center text-lg">{"press 'n' to start a new game!"}</p>
            </div>
        </section>
    }
}
