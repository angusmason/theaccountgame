use rand::prelude::SliceRandom;
use rand::thread_rng;

pub type Condition = (fn(&String) -> bool, String);
pub fn conditions() -> Vec<Condition> {
    vec![
        (
            |password: &String| password.len() >= 9,
            "Password must be at least nine characters long.",
        ),
        (
            |password: &String| password.chars().filter(char::is_uppercase).count() >= 12,
            "Password must contain at least twelve uppercase characters.",
        ),
        (
            |password: &String| password.chars().filter(char::is_lowercase).count() == 27,
            "Password must contain exactly twenty-seven lowercase characters.",
        ),
        (
            |password: &String| password.chars().filter(char::is_ascii_digit).count() >= 3,
            "Password must contain at least three digits.".to_string(),
        ),
        (
            |password: &String| {
                include_str!("anthem")
                    .split('\n')
                    .any(|line| password.contains(line))
            },
            "Password must contain a correctly punctuated line from the Australian national anthem.".to_string(),
        ),
        (
            |password: &String| !password.contains("Australia"),
            "Password may not contain the phrase 'Australia'.".to_string(),
        ),
        (
            |password: &String| !password.contains('s'),
            "Password may not contain the letter 's'.".to_string(),
        ),
        (
            |password: &String| false,
            {
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
                let clone = words.clone();
                let answer = clone.choose(&mut thread_rng()).unwrap();
                let mut feedback = String::new();
                words.shuffle(&mut thread_rng());
                let words = &words[..5];
                for word in words {
                    for (index, character) in word.chars().enumerate() {
                        let ansi = if answer.chars().position(|c| c == character) == Some(index) {
                            Colour::Green
                        } else if answer.contains(character) {
                            Colour::Yellow
                        } else {
                            Colour::Grey
                        }.into_ansi();
                        feedback.push_str(format!("\x1b[{ansi}m{character}\x1b[0m").as_str());
                    }
                    feedback.push('\n');
                }
                feedback.push_str("\nPassword must contain the answer to this Wordle.");
                feedback
            },
        ),
    ]
}
