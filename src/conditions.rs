use rand::prelude::SliceRandom;
use rand::thread_rng;
use yew::{html, Html};

pub type Condition = (Box<dyn Fn(&String) -> bool>, Html);
enum Colour {
    Grey,
    Yellow,
    Green,
}
pub fn conditions() -> Vec<Condition> {
    vec![
        (
            Box::new(|password: &String| password.len() >= 9),
            "Password must be at least nine characters long.".into(),
        ),
        (
            Box::new(|password: &String| password.chars().filter(|char| char.is_uppercase()).count() >= 5),
            "Password must contain at least five uppercase characters.".into(),
        ),
        (
            Box::new(|password: &String| password.chars().filter(|char| char.is_lowercase()).count() == 27),
            "Password must contain exactly twenty-seven lowercase characters.".into(),
        ),
        (
            Box::new(|password: &String| password.chars().filter(char::is_ascii_digit).count() >= 3),
            "Password must contain at least three digits.".into(),
        ),
        (
            Box::new(|password: &String| {
                include_str!("anthem")
                    .trim()
                    .split('\n')
                    .any(|line| password.contains(line))
            }),
            "Password must contain a correctly punctuated line from the Australian national anthem.".into(),
        ),
        (
            Box::new(|password: &String| !password.contains("Australia")),
            "Password may not contain the phrase 'Australia'.".into(),
        ),
        (
            Box::new(|password: &String| !password.contains('x')),
            "Password may not contain the letter 'x'.".into(),
        ),
        {

            let mut words: Vec<&str> = include_str!("words").split('\n').collect();
            let clone = words.clone();
            let answer = *clone.choose(&mut thread_rng()).unwrap();
            words.shuffle(&mut thread_rng());
            let words = &words[..5];
            let words = colour(words, answer);
            (
                Box::new(move |password: &String| password.contains(answer)),
                html! {
                    <div class="flex flex-col gap-4">
                        <p>{"Password must contain the answer to this Wordle."}</p>
                        <div class="flex">
                            <div class="text-white bg-slate-600 p-2">
                                {
                                    words.iter().map(|word| {
                                        html! {
                                            <div class="flex h-8">
                                                {word.iter().map(|(colour, character)| {
                                                    match colour {
                                                        Colour::Grey => html! {
                                                            <div class="bg-gray-500 w-8 grid place-content-center">
                                                                {character.to_string().to_uppercase()}
                                                            </div>
                                                        },
                                                        Colour::Yellow => html! {
                                                            <div class="bg-yellow-500 w-8 grid place-content-center">
                                                                {character.to_string().to_uppercase()}
                                                            </div>
                                                        },
                                                        Colour::Green => html! {
                                                            <div class="bg-green-500 w-8 grid place-content-center">
                                                                {character.to_string().to_uppercase()}
                                                            </div>
                                                        },
                                                    }
                                                }).collect::<Html>()}
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        </div>
                    </div>
                }
            )
        },
    ]
}

fn colour(words: &[&str], answer: &str) -> Vec<Vec<(Colour, char)>> {
    words
        .iter()
        .map(|word| {
            word.chars()
                .enumerate()
                .map(|(index, character)| {
                    (
                        'colour: {
                            if character == answer.chars().nth(index).unwrap() {
                                break 'colour Colour::Green;
                            }
                            let mut wrong_word = 0;
                            let mut wrong_guess = 0;
                            for (answer_index, answer_character) in answer.chars().enumerate() {
                                if answer_character == character
                                    && word.chars().nth(answer_index).unwrap() != character
                                {
                                    wrong_word += 1;
                                }
                                if (answer_index <= index)
                                    && (word.chars().nth(answer_index).unwrap() == character
                                        && answer_character != character)
                                {
                                    wrong_guess += 1;
                                }
                                if answer_index >= index {
                                    if wrong_guess == 0 {
                                        break;
                                    }
                                    if wrong_guess <= wrong_word {
                                        break 'colour Colour::Yellow;
                                    }
                                }
                            }
                            Colour::Grey
                        },
                        character,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
