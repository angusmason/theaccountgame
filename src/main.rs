#![warn(clippy::pedantic, clippy::nursery)]

mod conditions;

use conditions::conditions;
use rpassword::prompt_password;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <p>{ "John Doe: Building and breaking things" }</p>
                <p>{ "Jane Smith: The development process" }</p>
                <p>{ "Matt Miller: The Web 7.0" }</p>
                <p>{ "Tom Jerry: Mouseless development" }</p>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();

    let conditions = conditions();
    let password = 'outer: loop {
        let password = prompt_password("\nChoose a password: ")
            .unwrap()
            .trim()
            .to_string();
        for (condition, message) in &conditions {
            if !condition(&password) {
                println!("{message}");
                continue 'outer;
            }
        }
        println!("Password meets requirements.");
        break password;
    };

    loop {
        let password_confirmation = prompt_password("\nConfirm your password: ")
            .unwrap()
            .trim()
            .to_string();
        if password_confirmation == password {
            println!("\nPassword matches.");
            break;
        }
        println!("Password does not match.");
    }
}
