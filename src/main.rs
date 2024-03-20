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
        <main class="grid place-content-center h-full grid-cols-3">
            <div></div>
            <div class="flex flex-col divide-y">
                <div class="flex flex-col gap-4 p-4 relative z-0">
                    <h1 class="text-2xl font-semibold">
                        {"Create a password."}
                    </h1>
                    <input
                        type="password"
                        id="floating_password"
                        class="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent rounded-lg border-1 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                        placeholder=" "
                        {oninput}
                    />
                    <label
                        for="floating_password"
                        class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-gray-900 px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto start-1"
                    >{"Password"}</label>
                </div>
                <div class="flex flex-col gap-4 p-4">
                    <p class="text-1xl text-red-500 bg-red-200 rounded-xl p-4">
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
