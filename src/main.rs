#![warn(clippy::pedantic, clippy::nursery)]

mod conditions;

extern crate pancurses;

use conditions::conditions;
use rpassword::prompt_password;
use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();
    window.printw("Hello Rust");
    window.refresh();
    window.getch();
    endwin();
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
