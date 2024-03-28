use chrono::Local;
use rand::thread_rng;
use rand::{prelude::SliceRandom, Rng};
use yew::{html, Html};

pub type Condition = (Box<dyn Fn(&String, &String) -> bool>, Html);
enum Colour {
    Grey,
    Yellow,
    Green,
}

#[allow(clippy::too_many_lines)]
pub fn conditions() -> Vec<Condition> {
    let numbers: Vec<_> = include_str!("numbers").trim().split('\n').collect();
    let vec = vec![
        (
            Box::new(
                |_username: &String, password: &String|
                    !password.to_lowercase().contains("bean")
            ) as Box<dyn Fn(&String, &String) -> bool>,
            "Password may not contain the phrase 'bean'.".into(),
        ),
        {
            let number = thread_rng().gen_range(3..=6);
            (
                Box::new(move |_username: &String, password: &String|
                    password
                        .chars()
                        .filter(|char| char.is_uppercase()).count() >= number
                ),
                format!(
                    "Password must contain at least {} uppercase characters.",
                    numbers[number]
                ).into(),
            )
        },
        {
            let number = thread_rng().gen_range(3..=6);
            (
                Box::new(move |_username: &String, password: &String|
                    password
                        .chars()
                        .filter(char::is_ascii_digit).count() >= number
                ),
                format!(
                    "Password must contain at least {} digits.",
                    numbers[number]
                ).into(),
            )
        },
        (
            Box::new(|_username: &String, password: &String| {
                include_str!("anthem")
                    .trim()
                    .split('\n')
                    .any(|line| password.contains(line))
            }),
            "Password must contain a correctly punctuated line from the Australian national anthem."
                .into(),
            ),
            (
                Box::new(
                    |_username: &String, password: &String|
                        !password.contains("Australia")
                ),
                "Password may not contain the phrase 'Australia'.".into(),
            ),
            (
                Box::new(
                    |_username: &String, password: &String|
                        password.contains('🚡')
                ),
                "Password must contain the aerial tramway emoji.".into(),
            ),
            (
                Box::new(
                    |_username: &String, password: &String|
                        password.to_lowercase().contains('')
                ),
                "Password must contain the Apple logo.".into(),
            ),
            {
            let mut words: Vec<&str> = include_str!("words").split('\n').collect();
            let clone = words.clone();
            let answer = *clone.choose(&mut thread_rng()).unwrap();
            words.shuffle(&mut thread_rng());
            let words = &words[..5];
            let words = colour(words, answer);
            (
                Box::new(
                    move |_username: &String, password: &String|
                        password.to_lowercase().contains(answer)
                ),
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
                                                            <div
                                                                class="bg-gray-500 w-8 grid
                                                                    place-content-center"
                                                            >
                                                                {character.to_string().to_uppercase()}
                                                            </div>
                                                        },
                                                        Colour::Yellow => html! {
                                                            <div
                                                                class="bg-yellow-500 w-8 grid
                                                                    place-content-center"
                                                            >
                                                                {character.to_string().to_uppercase()}
                                                            </div>
                                                        },
                                                        Colour::Green => html! {
                                                            <div
                                                                class="bg-green-500 w-8 grid
                                                                    place-content-center"
                                                            >
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
        (
            Box::new(
                |_username: &String, password: &String|
                    password.contains(password.len().to_string().as_str())
            ),
            "Password must contain its length.".into(),
        ),
        (
            Box::new(|username: &String, password: &String| password.contains(&username.chars().rev().collect::<String>())),
            "Password must contain the username reversed.".into(),
        ),
        {
            let number = thread_rng().gen_range(29..=39);
            (
                Box::new(move |_username: &String, password: &String| {
                    password
                        .chars()
                        .filter_map(|char| char.to_string().parse::<usize>().ok())
                        .sum::<usize>()
                        == number
                }),
                format!("Digits in password must sum to {}.", numbers[number]).into(),
            )
        },
        {
            let [r, g, b] = (0..3)
                .map(|_| thread_rng().gen_range(0..=0xff))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let hex = format!("{r:02x}{g:02x}{b:02x}");
            (
                {
                    let hex = hex.clone();
                    Box::new(
                        move |_username: &String, password: &String|
                            password.to_lowercase().contains(&hex)
                    )
                },
                html! {
                    <div class="flex flex-col gap-4">
                        <p>{"Password must contain the 24-bit hexadecimal colour of this box."}</p>
                        <div
                            class="w-32 h-32 border-slate-600 border-8"
                            style={format!("background-color: #{hex}")}
                        />
                    </div>
                },
            )
        },
        (
            Box::new(
                |_username: &String, password: &String|
                    password.to_lowercase().contains("blue")
            ),
            "Password must contain my favourite colour.".into(),
        ),
        (
            Box::new(
                |_username: &String, password: &String|
                    password.contains(&Local::now().format("%-H:%M").to_string())
            ),
            "Password must contain the current time in the format HH:MM.".into(),
        ),
        (
            Box::new(
                |username: &String, password: &String|
                    password.contains(&username.chars().rev().collect::<String>())
            ),
            "Password must contain the username reversed.".into(),
        ),
        {
            let number = thread_rng().gen_range(46..=58);
            (
                Box::new(move |_username: &String, password: &String|
                    password
                    .chars()
                    .filter(|char| char.is_lowercase()).count() == number
                ),
                format!(
                    "Password must contain exactly {} lowercase characters.",
                    numbers[number]
                ).into(),
            )
        },
        (
            Box::new(
                |_username: &String, password: &String|
                    *password == password.chars().rev().collect::<String>()
            ),
            "Password must be a palindrome.".into(),
        ),
        {
            let (riddle, answer) = [
                ("very good riddle", "answer"),
                ("another very good riddle", "answer")
            ].choose(&mut thread_rng()).unwrap();
            (
                Box::new(
                    move |_username: &String, password: &String|
                        password.contains(answer)
                ) as Box<dyn Fn(&String, &String) -> bool>,
                html! {
                    <div class="flex flex-col gap-4">
                        <p>{"Password must contain the answer to this riddle:"}</p>
                        <p>{*riddle}</p>
                    </div>
                }
            )
        }
    ];
    vec
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
