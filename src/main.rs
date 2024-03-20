#![warn(clippy::pedantic, clippy::nursery)]

use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Html, InputEvent, TargetCast};

#[function_component]
fn App() -> Html {
    let input = use_state(String::new);
    let oninput = move |event: InputEvent| {
        input.set(event.target_dyn_into::<HtmlInputElement>().unwrap().value());
    };

    html! {
        <main class="grid place-content-center h-full">
            <div class="flex flex-col divide-y">
                <div class="flex flex-col gap-4 p-4">
                    <h1 class="text-3xl">
                        {"Please choose a password."}
                    </h1>
                    <input
                        type="password"
                        placeholder="Password"
                        class="rounded p-2"
                        {oninput}
                    />
                </div>
                <div class="flex flex-col gap-4 p-4">
                    <h1 class="text-2xl">
                        {"Things wrong with your password:"}
                    </h1>
                </div>
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
