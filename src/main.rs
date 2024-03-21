#![warn(clippy::pedantic, clippy::nursery)]

mod conditions;

use crate::conditions::conditions;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_memo, use_state, Html, InputEvent, Renderer, TargetCast};

#[function_component]
fn App() -> Html {
    // State to store the password
    let password = use_state(String::new);
    // Handler for the input event
    let oninput = {
        // Clone the password state so we can move it into the closure
        let password = password.clone();
        move |event: InputEvent| {
            // Get the target of the event and dynamically cast it to an HtmlInputElement, then get
            // the value of the input and set the password state to it
            password.set(
                event
                    .target_dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .replace('\n', ""),
            );
        }
    };
    // Generate the conditions
    let conditions = use_memo((), |()| conditions());
    // Filter the conditions and get the messages for the ones that are wrong
    // Collect them into a Vec
    let wrong = conditions
        .iter()
        .find_map(|(condition, message)| (!condition(&password)).then_some(message).cloned());

    // Return some HTML
    html! {
        <main class="flex sm:justify-center">
            <div></div>
            <div class="flex flex-col divide-y w-full sm:w-1/3">
                <div class="flex flex-col gap-4 p-4">
                    <h1 class="text-2xl font-semibold">
                        {"Create a password."}
                    </h1>
                    <div class="relative">
                        <input
                            type="password"
                            id="password"
                            placeholder=" "
                            class="block px-2.5 pb-2.5 pt-4 w-full text-sm rounded-lg border
                                border-gray-300 focus:ring-0 focus:border-blue-600 peer"
                            {oninput}
                        />
                        <label
                            for="password"
                            class="absolute text-gray-400 duration-300 top-1 scale-75 left-2
                                -translate-y-4 bg-white px-2 peer-focus:text-blue-600
                                origin-left peer-placeholder-shown:scale-100 peer-focus:top-1
                                peer-placeholder-shown:top-1/2 peer-focus:scale-75
                                peer-focus:-translate-y-4 peer-placeholder-shown:-translate-y-3"
                        >
                            {"Password"}
                        </label>
                    </div>
                </div>
                <div class="flex flex-col gap-4 p-4">
                    {
                        // Map the wrong message to a HTML element
                        // If it was Some, it will map to a paragraph with the message
                        // If it was None or if the password is empty, it will map to nothing
                        (!password.is_empty())
                            .then_some(())
                            .and(wrong.map(|message| html! {
                                <p class="text-1xl text-red-500 bg-red-200 rounded-xl p-4">
                                    {message}
                                </p>
                            }))
                    }
                </div>
            </div>
        </main>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
