use rand::prelude::SliceRandom;
use rand::thread_rng;

pub type Condition = (fn(&String) -> bool, &'static str);
pub fn conditions() -> Vec<Condition> {
    vec![
        (
            |password: &String| password.len() >= 10,
            "Password must be at least ten characters long.",
        ),
        (
            |password: &String| password.chars().any(char::is_uppercase),
            "Password must contain at least one uppercase character.",
        ),
        (
            |password: &String| password.chars().filter(char::is_ascii_digit).count() >= 3,
            "Password must contain at least three digits.",
        ),
        (
            |password: &String| {
                include_str!("anthem")
                    .split('\n')
                    .any(|line| password.contains(line))
            },
            "Password must contain a correctly punctuated line from the Australian national anthem.",
        ),
        (
            |password: &String| !password.contains("Australia"),
            "Password may not contain the phrase 'Australia'.",
        ),
        (
            |password: &String| !password.contains('s'),
            "Password may not contain the letter 's'.",
        ),
        (
            |password: &String| {
                enum Colour {
                    Grey, Yellow, Green
                }
                impl Colour {
                    const fn into_ansi(self) -> &'static str {
                        match self {
                            Self::Grey => "30",
                            Self::Yellow => "33",
                            Self::Green => "32",
                        }
                    }
                }
                let mut words: Vec<&str> = include_str!("words").split('\n').collect();
                let answer = words.choose(&mut thread_rng()).unwrap();
                words.shuffle(&mut thread_rng());
                let words = &words[..5];
                for word in words {
                    for character in word.chars() {
                        if answer.contains(character) {
                            Colour::Yellow
                        }
                    }
                }
                true
            },
            "Password must be at least ten characters long.",
        ),
    ]
}
