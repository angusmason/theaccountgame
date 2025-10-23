#![warn(clippy::pedantic, clippy::nursery)]

mod conditions;

use crate::conditions::conditions;
use chrono::Local;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_effect, use_memo, use_state, virtual_dom::VNode, Html,
    InputEvent, Properties, Renderer, TargetCast,
};

#[derive(Properties, PartialEq)]
struct ErrorProps {
    message: VNode,
}

#[function_component]
fn Error(props: &ErrorProps) -> Html {
    html! {
        <p class="p-4 text-red-500 bg-red-200 border border-red-500 text-1xl rounded-xl">
            {props.message.clone()}
        </p>
    }
}

#[function_component]
fn App() -> Html {
    // State to store the username
    let username = use_state(String::new);
    // State to store the password
    let password = use_state(String::new);
    // State to store the confirmation password
    let confirm = use_state(String::new);
    let won = use_state(|| false);
    // Generate the conditions
    let conditions = use_memo((), |()| conditions());
    let discovered = use_state(|| conditions.iter().map(|_| false).collect::<Vec<_>>());
    let time = use_state(|| Local::now().to_rfc3339());
    use_effect({
        let conditions = conditions.clone();
        let username = username.clone();
        let password = password.clone();
        let confirm = confirm.clone();
        move || {
            let interval = gloo_timers::callback::Interval::new(1000, move || {
                time.set(Local::now().to_rfc3339());
                if conditions
                    .iter()
                    .enumerate()
                    .find_map(|(index, (condition, message))| {
                        (!condition(&username, &password)).then_some((message.clone(), index))
                    })
                    .is_some()
                {
                    confirm.set(String::new());
                }
            });
            move || drop(interval)
        }
    });
    // Find the condition that is not met and map it to the message
    let (wrong, wrong_index) = conditions
        .iter()
        .enumerate()
        .find_map(|(index, (condition, message))| {
            (!condition(&username, &password)).then_some((message.clone(), index))
        })
        .unzip();
    let username_oninput = {
        // Clone states so we can move them into the closure
        let username = username.clone();
        move |event: InputEvent| {
            // Get the target of the event and dynamically cast it to an HtmlInputElement, then get
            // the value of the input and set the username state to it
            username.set(event.target_dyn_into::<HtmlInputElement>().unwrap().value());
        }
    };
    let password_oninput = {
        // Clone states so we can move them into the closure
        let password = password.clone();
        let confirm = confirm.clone();
        let discovered = discovered.clone();
        move |event: InputEvent| {
            // Get the target of the event and dynamically cast it to an HtmlInputElement, then get
            // the value of the input and set the password state to it
            password.set(event.target_dyn_into::<HtmlInputElement>().unwrap().value());
            confirm.set(String::new());
            // Mark the unsatisfied condition as discovered
            if let Some(index) = wrong_index {
                let mut cloned = discovered.to_vec();
                cloned[index] = true;
                discovered.set(cloned);
            }
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

    let submit = {
        let won = won.clone();
        let confirm = confirm.clone();
        let password = password.clone();
        move |_| {
            won.set((confirm == password) && (!password.is_empty()));
        }
    };

    // Return some HTML
    html! {
        <main class="flex justify-center h-full grow">
            <div
                class="flex flex-col items-center justify-center w-full h-full max-w-md gap-4 px-4"
            >
                {
                    if *won {
                        html! {
                            <div class="relative flex flex-col w-full gap-4">
                                <p class="p-4 text-lg text-red-500 bg-red-200 border border-red-500 rounded-xl">
                                    {"This password is already taken. Please choose another."}
                                </p>
                            </div>
                        }
                    } else {
                        html! {
                            <>
                                <div class="relative flex flex-col w-full gap-4">
                                    <h1 class="text-2xl font-semibold">
                                        {"Create an account."}
                                    </h1>
                                    <input
                                        oninput={username_oninput}
                                        placeholder="Username"
                                        id="username"
                                        autocomplete="off"
                                        class="w-full p-3 text-lg transition-transform bg-white border border-gray-700 rounded-xl focus:outline-none"
                                    />
                                    <input
                                        oninput={password_oninput}
                                        placeholder="Password"
                                        type="password"
                                        id="password"
                                        autocomplete="off"
                                        class="w-full p-3 text-lg transition-transform bg-white border border-gray-700 rounded-xl focus:outline-none"
                                    />
                                </div>
                                <div class="relative flex flex-col w-full gap-4">
                                    <div class={classes!(
                                        "flex", "flex-col", "gap-4",
                                        wrong.is_some().then_some("hidden")
                                    )}>
                                        <input
                                            oninput={confirm_oninput}
                                            placeholder="Confirm password"
                                            type="password"
                                            id="confirm"
                                            autocomplete="off"
                                            class="w-full p-3 text-lg transition-transform bg-white border border-gray-700 rounded-xl focus:outline-none"
                                        />
                                    </div>
                                    <button
                                        disabled={(confirm != password) || (password.is_empty())}
                                        class="p-2 transition bg-white border border-gray-700 disabled:opacity-25 disabled:pointer-events-none rounded-xl hover:bg-gray-200"
                                        onclick={submit}
                                    >
                                        {"Submit"}
                                    </button>
                                    <div class="absolute inset-x-0 flex flex-col gap-4 py-4 top-full">
                                        {
                                            // Map the wrong message to a HTML element
                                            // If it was Some, it will map to a paragraph with the message
                                            // If it was None or if the password is empty, it will map to nothing
                                            (!password.is_empty())
                                                .then_some(())
                                                .and(wrong.as_ref().map(|message| html! {
                                                    <Error message={message} />
                                                }))
                                        }
                                        {
                                            // Filter through the conditions and map the wrong ones to a HTML
                                            // element
                                            conditions
                                                .iter()
                                                .enumerate()
                                                .filter_map(|(index, (condition, message))|
                                                    (
                                                        discovered[index]
                                                            && !condition(&username, &password)
                                                            && wrong_index != Some(index)
                                                            && !password.is_empty()
                                                    )
                                                        .then_some(html! {
                                                            <Error message={message} />
                                                        })
                                                ).collect::<Vec<_>>()
                                        }
                                    </div>
                                    <div class="absolute inset-x-0 flex flex-col gap-4 pt-4 top-full">
                                        {
                                            (confirm != password && !confirm.is_empty())
                                                .then_some(())
                                                .map(|()| html! {
                                                    <Error message="Passwords do not match." />
                                                })
                                        }
                                    </div>
                                </div>
                            </>
                        }
                    }
                }
            </div>
        </main>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
