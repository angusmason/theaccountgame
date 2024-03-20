#![warn(clippy::pedantic, clippy::nursery)]

use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_state, Html, InputEvent, TargetCast};

#[function_component]
fn App() -> Html {
    let input = use_state(String::new);
    let oninput = {
        let input = input.clone();
        move |event: InputEvent| {
            input.set(event.target_dyn_into::<HtmlInputElement>().unwrap().value());
        }
    };
    html! {
        <div class={classes!("")}>
            {"Please choose a password."}
            <input type="password" {oninput}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
