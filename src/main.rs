#![warn(clippy::pedantic, clippy::nursery)]

mod conditions;

use crate::conditions::conditions;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_memo, use_state, Callback, Html, InputEvent, Properties,
    Renderer, TargetCast,
};

#[derive(Properties, PartialEq)]
struct InputProps {
    oninput: Callback<InputEvent>,
    id: String,
    placeholder: String,
    value: String,
}

#[function_component]
fn Input(props: &InputProps) -> Html {
    let oninput = props.oninput.clone();
    html! {
        <div class="relative">
            <input
                type="password"
                id={props.id.clone()}
                placeholder=" "
                autocomplete="off"
                value={props.value.clone()}
                class="block px-2.5 pb-2.5 pt-4 w-full text-1xl rounded-lg border border-gray-300
                    focus:ring-0 focus:border-blue-600 peer"
                {oninput}
            />
            <label
                for={props.id.clone()}
                class="absolute text-gray-400 duration-300 top-1 scale-75 left-2 -translate-y-4
                    bg-white px-2 peer-focus:text-blue-600 origin-left
                    peer-placeholder-shown:scale-100 peer-focus:top-1 peer-placeholder-shown:top-1/2
                    peer-focus:scale-75 peer-focus:-translate-y-4
                    peer-placeholder-shown:-translate-y-3"
            >
                {props.placeholder.clone()}
            </label>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    // State to store the password
    let password = use_state(String::new);
    // State to store the confirmation password
    let confirm = use_state(String::new);
    // Handler for the input event
    let password_oninput = {
        // Clone the password state so we can move it into the closure
        let password = password.clone();
        let confirm = confirm.clone();
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
            confirm.set(String::new());
        }
    };
    let confirm_oninput = {
        // Clone the password state so we can move it into the closure
        let confirm = confirm.clone();
        move |event: InputEvent| {
            // Get the target of the event and dynamically cast it to an HtmlInputElement, then get
            // the value of the input and set the password state to it
            confirm.set(
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
        <main class="flex justify-center h-full">
            <div
                class="flex flex-col items-center justify-center h-full max-w-md w-full px-4 gap-4"
            >
                <div class="flex flex-col gap-4 relative w-full">
                    <h1 class="text-2xl font-semibold">
                        {"Create a password."}
                    </h1>
                    <Input
                        oninput={password_oninput}
                        id="password"
                        placeholder="Password"
                        value={(*password).clone()}
                    />
                    <div class="flex flex-col gap-4 absolute top-full pt-4 inset-x-0">
                        {
                            // Map the wrong message to a HTML element
                            // If it was Some, it will map to a paragraph with the message
                            // If it was None or if the password is empty, it will map to nothing
                            (!password.is_empty())
                                .then_some(())
                                .and(wrong.as_ref().map(|message| html! {
                                    <p class="text-1xl text-red-500 bg-red-200 rounded-xl p-4">
                                        {message}
                                    </p>
                                }))
                        }

                    </div>
                </div>
                <div class="flex flex-col gap-4 relative w-full">
                    <div class={classes!(
                        "transition", "duration-300", "flex", "flex-col", "gap-4",
                        wrong.is_some().then_some("opacity-0")
                    )}>
                        <Input
                            oninput={confirm_oninput}
                            id="confirm"
                            placeholder="Confirm password"
                            value={(*confirm).clone()}
                        />
                        <button
                            disabled={confirm != password}
                            class="disabled:opacity-20 border-gray-700 border p-2 rounded-xl
                                bg-w hover:bg-gray-200 transition"
                        >
                            {"Submit"}
                        </button>
                    </div>
                    <div class="flex flex-col gap-4 absolute top-full pt-4 inset-x-0">
                        {
                            (confirm != password && !confirm.is_empty())
                                .then_some(())
                                .map(|()| html! {
                                    <p class="text-1xl text-red-500 bg-red-200 rounded-xl p-4">
                                        {"Passwords do not match."}
                                    </p>
                                })
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
