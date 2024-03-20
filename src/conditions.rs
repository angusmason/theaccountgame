use rand::prelude::SliceRandom;
use rand::thread_rng;

pub type Condition = (Box<dyn Fn(&String) -> bool>, String);
pub fn conditions() -> Vec<Condition> {
    vec![
        (
            Box::new(|password: &String| password.len() >= 9),
            "Password must be at least nine characters long.".to_string(),
        ),
        (
            Box::new(|password: &String| password.chars().filter(|char| char.is_uppercase()).count() >= 5),
            "Password must contain at least five uppercase characters.".to_string(),
        ),
        (
            Box::new(|password: &String| password.chars().filter(|char| char.is_lowercase()).count() == 27),
            "Password must contain exactly twenty-seven lowercase characters.".to_string(),
        ),
        (
            Box::new(|password: &String| password.chars().filter(char::is_ascii_digit).count() >= 3),
            "Password must contain at least three digits.".to_string(),
        ),
        (
            Box::new(|password: &String| {
                include_str!("anthem")
                    .trim()
                    .split('\n')
                    .any(|line| password.contains(line))
            }),
            "Password must contain a correctly punctuated line from the Australian national anthem.".to_string(),
        ),
        (
            Box::new(|password: &String| !password.contains("Australia")),
            "Password may not contain the phrase 'Australia'.".to_string(),
        ),
        (
            Box::new(|password: &String| !password.contains('s')),
            "Password may not contain the letter 's'.".to_string(),
        ),
        // {
        //     enum Colour {
        //         Grey, Yellow, Green
        //     }
        //     impl Colour {
        //         const fn into_ansi(self) -> &'static str {
        //             match self {
        //                 Self::Grey => "30",
        //                 Self::Yellow => "33",
        //                 Self::Green => "32",
        //             }
        //         }
        //     }
        //     let mut words: Vec<&str> = include_str!("words").split('\n').collect();
        //     let clone = words.clone();
        //     let answer = (*clone.choose(&mut thread_rng()).unwrap()).to_string();
        //     let mut feedback = String::new();
        //     words.shuffle(&mut thread_rng());
        //     let words = &words[..5];
        //     for word in words {
        //         for (index, character) in word.chars().enumerate() {
        //             let ansi = 'colour: {
        //                 if character == answer.chars().nth(index).unwrap() {
        //                     break 'colour Colour::Green;
        //                 }

        //                 let mut wrong_word = 0;
        //                 let mut wrong_guess = 0;
        //                 for (answer_index, answer_character) in answer.chars().enumerate() {
        //                     if answer_character == character && word.chars().nth(answer_index).unwrap() != character {
        //                         wrong_word += 1;
        //                     }
        //                     if (answer_index <= index) && (word.chars().nth(answer_index).unwrap() == character && answer_character != character) {
        //                         wrong_guess += 1;
        //                     }
        //                     if answer_index >= index {
        //                         if wrong_guess == 0 {
        //                             break;
        //                         }
        //                         if wrong_guess <= wrong_word {
        //                             break 'colour Colour::Yellow;
        //                         }
        //                     }
        //                 }
        //                 Colour::Grey
        //             }.into_ansi();
        //             feedback.push_str(&format!("\x1b[{ansi}m{character}\x1b[0m"));

        //         }
        //         feedback.push('\n');
        //     }
        //     feedback.push_str("\nPassword must contain the answer to this Wordle.");
        //     (
        //         Box::new(move |password: &String| password.contains(&answer)),
        //         feedback,
        //     )
        // },
    ]
}
