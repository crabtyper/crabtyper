use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StatusLineProps {
    pub mode: String,
    pub timer: String,
    pub wpm: u32,
    pub lang: String,
    pub progress: String,
}

#[function_component(Statusline)]
pub fn statusline(props: &StatusLineProps) -> Html {
    let mode_class = {
        if &props.mode == "NORMAL" {
            "bg-green"
        } else {
            "bg-blue"
        }
    };

    html! {
        <>
            <div class="flex flex-row justify-between w-full bg-gray items-center font-bold text-white">
                <div class="flex flex-row gap-4 items-center">
                    <p class={classes!(String::from("text-black px-4 py-1"), mode_class)}>
                        {&props.mode}
                    </p>
                    <div class="flex items-center gap-2">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                          <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd" />
                        </svg>
                        <p>{&props.timer}</p>
                    </div>
                    <p>{format!("WPM: {}", &props.wpm)}</p>
                </div>
                <div class="flex flew-row gap-4 items-center">
                    <p>{&props.lang}</p>
                    <p class={classes!(String::from("text-black px-4 py-1 min-w-[5rem] text-center"), mode_class)}>
                        {&props.progress}
                    </p>
                </div>
            </div>
        </>
    }
}
