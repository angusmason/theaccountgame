#![warn(clippy::pedantic, clippy::nursery)]

mod conditions;

use crate::conditions::conditions;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::{function_component, html, use_state, Html, InputEvent, Renderer, TargetCast};

#[function_component]
fn App() -> Html {
    // State to store the password
    let password = use_state(String::new);
    // Handler for the input event
    let oninput = {
        // Clone the password state so we can move it into the closure
        let password = password.clone();
        move |event: InputEvent| {
            // Get the target of the event and dynamically cast it to an HtmlInputElement, then get the
            // value of the input and set the password state to it
            password.set(
                event
                    .target_dyn_into::<HtmlTextAreaElement>()
                    .unwrap()
                    .value()
                    .replace('\n', ""),
            );
        }
    };
    // Generate the conditions
    let conditions = conditions();
    // Filter the conditions and get the messages for the ones that are wrong
    // Collect them into a Vec
    let wrong = conditions
        .iter()
        .find_map(|(condition, message)| (!condition(&password)).then_some(message).cloned());

    // Return some HTML
    html! {
        <main class="grid place-content-center h-full grid-cols-3">
            <div></div>
            <div class="flex flex-col divide-y">
                <div class="flex flex-col gap-4 p-4">
                    <h1 class="text-3xl">
                        {"Please choose a password."}
                    </h1>
                    <textarea
                        type="password"
                        placeholder="Password"
                        class="rounded p-2"
                        value={(*password).clone()}
                        {oninput}
                    />
                </div>
                <div class="flex flex-col gap-4 p-4">
                    <p class="text-2xl text-red-500 bg-red-200 rounded-xl p-4">
                        {
                            // Map the wrong message to a HTML element
                            // If it was Some, it will map to a paragraph with the message
                            // If it was None, it will map to nothing and not render anything
                            wrong.map(|message| html! { <p>{message}</p> })
                        }
                    </p>
                </div>
            </div>
        </main>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
