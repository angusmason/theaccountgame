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
    let discovered = use_state(|| conditions.iter().map(|_| false).collect::<Vec<_>>());
    // Find the condition that is not met and map it to the message
    let (wrong, wrong_index) = conditions
        .iter()
        .enumerate()
        .find_map(|(index, (condition, message))| {
            (!condition(&password)).then_some((message.clone(), index))
        })
        .unzip();

    // If there is a wrong condition, mark it as discovered
    if let Some(index) = wrong_index {
        let mut cloned = discovered.to_vec();
        cloned[index] = true;
        discovered.set(cloned);
    }

    // Return some HTML
    html! {
        <main class="flex justify-center h-full">
            <div class="flex flex-col items-center justify-center h-full max-w-md w-full px-4">
                <div class="flex flex-col gap-4 relative w-full">
                    <h1 class="text-2xl font-semibold">
                        {"Create a password."}
                    </h1>
                    // <div class="relative">
                    //     <input
                    //         id="username"
                    //         placeholder=" "
                    //         autocomplete="off"
                    //         class="block px-2.5 pb-2.5 pt-4 w-full text-1xl rounded-lg border
                    //             border-gray-300 focus:ring-0 focus:border-blue-600 peer"
                    //     />
                    //     <label
                    //         for="username"
                    //         class="absolute text-gray-400 duration-300 top-1 scale-75 left-2
                    //             -translate-y-4 bg-white px-2 peer-focus:text-blue-600
                    //             origin-left peer-placeholder-shown:scale-100 peer-focus:top-1
                    //             peer-placeholder-shown:top-1/2 peer-focus:scale-75
                    //             peer-focus:-translate-y-4 peer-placeholder-shown:-translate-y-3"
                    //     >
                    //         {"Username"}
                    //     </label>
                    // </div>
                    <div class="relative">
                        <input
                            type="password"
                            id="password"
                            placeholder=" "
                            autocomplete="off"
                            class="block px-2.5 pb-2.5 pt-4 w-full text-1xl rounded-lg border
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
                    <div class="flex flex-col gap-4 absolute top-full pt-4 inset-x-0">
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
            </div>
        </main>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
