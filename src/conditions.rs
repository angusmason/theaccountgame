use std::iter::successors;

use chrono::Local;
use rand::thread_rng;
use rand::{prelude::SliceRandom, Rng};
use yew::virtual_dom::VNode;
use yew::{classes, html, Html};

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
                Box::new(|_username: &String, password: &String| {
                    !password.to_lowercase().contains("bean")
                }) as Box<dyn Fn(&String, &String) -> bool>,
                "Password may not contain the phrase 'bean'.".into(),
            ),
            {
                let number = thread_rng().gen_range(3..=6);
                (
                    Box::new(move |_username, password|
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
                    Box::new(move |_username, password|
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
                Box::new(|_username, password| {
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
                        |_username, password|
                            !password.contains("Australia")
                    ),
                    "Password may not contain the phrase 'Australia'.".into(),
                ),
                (
                    Box::new(
                        |_username, password|
                            password.contains('\u{1F6A1}')
                    ),
                    "Password must contain the aerial tramway emoji.".into(),
                ),
                (
                    Box::new(
                        |_username, password|
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
                        move |_username, password|
                            password.to_lowercase().contains(answer)
                    ),
                    html! {
                        <div class="flex flex-col gap-4">
                            <p>{"Password must contain the answer to this Wordle."}</p>
                            <div class="flex">
                                <div class="p-2 text-white bg-slate-600">
                                    {
                                        words.iter().map(|word| {
                                            html! {
                                                <div class="flex h-8">
                                                    {word.iter().map(|(colour, character)| {
                                                        match colour {
                                                            Colour::Grey => html! {
                                                                <div
                                                                    class="grid w-8 bg-gray-500 // place-content-center"
                                                                >
                                                                    {character.to_string().to_uppercase()}
                                                                </div>
                                                            },
                                                            Colour::Yellow => html! {
                                                                <div
                                                                    class="grid w-8 bg-yellow-500 // place-content-center"
                                                                >
                                                                    {character.to_string().to_uppercase()}
                                                                </div>
                                                            },
                                                            Colour::Green => html! {
                                                                <div
                                                                    class="grid w-8 bg-green-500 // place-content-center"
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
                    |_username, password|
                        password.contains(password.len().to_string().as_str())
                ),
                "Password must contain its length.".into(),
            ),
            (
                Box::new(|username, password| password.contains(&username.chars().rev().collect::<String>())),
                "Password must contain the username reversed.".into(),
            ),
            {
                let number = thread_rng().gen_range(58..=68);
                (
                    Box::new(move |_username, password| {
                        password
                            .chars()
                            .filter_map(|char| char.to_string().parse::<usize>().ok())
                            .sum::<usize>()
                            == number
                    }),
                    format!("Digits in password must sum to {}.", numbers[number]).into(),
                )
            },
                        (
                Box::new(
                    |_username, password|
                        password.to_lowercase().contains("blue")
                ),
                "Password must contain my favourite colour.".into(),
            ),
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
                            move |_username, password|
                                password.to_lowercase().contains(&hex)
                        )
                    },
                    html! {
                        <div class="flex flex-col gap-4">
                            <p>{"Password must contain the 24-bit hexadecimal colour of this box."}</p>
                            <div
                                class="w-32 h-32 border-8 border-slate-600"
                                style={format!("background-color: #{hex}")}
                            />
                        </div>
                    },
                )
            },
            {
                let number = thread_rng().gen_range(46..=58);
                (
                    Box::new(move |_username, password|
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
            {
                let (riddle, answer) = [
                    ("What do you call a person that's struggling to set a password? An _____.", "idiot")
                    ].choose(&mut thread_rng()).unwrap();
                    (
                        Box::new(
                            move |_username, password|
                            password.to_lowercase().contains(answer)
                        ),
                        html! {
                            <div class="flex flex-col gap-4">
                            <p>{"Password must contain the answer to this riddle:"}</p>
                            <p>{*riddle}</p>
                            </div>
                        }
                    )
            },
            (
                Box::new(|_username, password| ('\u{1F3FB}'..='\u{1F3FF}').all(|char| password.contains(char))),
                "Password must be ethnically diverse. 👍".into()
            ),
            {
                    const MAZE_SIZE: u32 = 20;
                    const BORDER_WIDTH: u32 = 1;
                    type Cell = (Position, bool);
                    type Position = (u32, u32);
                    fn neighbours((x, y): Position) -> Vec<Position> {
                        vec![
                            (x, y.wrapping_sub(1)),
                            (x.wrapping_add(1), y),
                            (x, y.wrapping_add(1)),
                            (x.wrapping_sub(1), y),
                            ]
                        }
                        #[repr(u8)]
                        enum Direction {
                            Up = b'R',
                            Right = b'D',
                            Down = b'L',
                            Left = b'U',
                        }

                        let mut maze: Vec<_> = (0..MAZE_SIZE)
                        .flat_map(|y| (0..MAZE_SIZE).map(move |x| ((x, y), false) as Cell))
                        .collect();
                    let mut stack = Vec::new();
                    let mut paths = Vec::new();
                    let cell = (0, 0);
                    let (_, visited) = maze
                    .iter_mut()
                    .find(|(position, _)| *position == cell)
                    .unwrap();
                *visited = true;
                stack.push(cell);
                while let Some(cell) = stack.pop() {
                    let neighbours: Vec<_> = neighbours(cell)
                    .into_iter()
                    .filter_map(|neighbour| {
                        maze.iter().find_map(|(position, visited)| {
                            (*position == neighbour && !visited).then_some(*position)
                        })
                    })
                    .collect();
                if let Some(neighbour) = neighbours.choose(&mut thread_rng()) {
                    stack.push(cell);
                    paths.push((cell, *neighbour));
                    let (_, visited) = maze
                    .iter_mut()
                    .find(|(position, _)| *position == *neighbour)
                    .unwrap();
                *visited = true;
                stack.push(*neighbour);
            }
        }
        let (solution, start, goal) = {
            let maze: Vec<_> = maze.iter().map(|(position, _)| position).copied().collect();
            let cell = *maze.choose(&mut thread_rng()).unwrap();
            let goal = *maze.choose(&mut thread_rng()).unwrap();
            let mut queue = vec![cell];
            let mut explored = vec![cell];
            let mut links = Vec::new();
            while let Some(cell) = queue.pop() {
                if cell == goal {
                    break;
                }
                for neighbour in neighbours(cell) {
                    if maze.contains(&neighbour)
                    && !explored.contains(&neighbour)
                    && paths.iter().any(|(a, b)| {
                        *a == cell && *b == neighbour || *a == neighbour && *b == cell
                    })
                    {
                        queue.push(neighbour);
                        links.push((neighbour, cell));
                        explored.push(neighbour);
                    }
                }
            }
            let path: Vec<_> = successors(Some(goal), move |&cell| {
                links
                .iter()
                .find_map(|(to, from)| (*to == cell).then_some(*from))
            })
            .collect();
        let path: Vec<_> = path.iter().copied().rev().collect();
        (path, cell, goal)
    };
    let solution: String = solution
    .windows(2)
    .map(|window| {
        let [from, to] = window else { unreachable!() };
        (if from.0 < to.0 {
            Direction::Right
        } else if from.0 > to.0 {
            Direction::Left
        } else if from.1 < to.1 {
            Direction::Down
        } else {
            Direction::Up
        }) as u8 as char
    })
    .collect();
    let maze = (0..MAZE_SIZE)
    .flat_map(|y| {
        let paths = paths.clone();
        (0..MAZE_SIZE).map(move |x| {
            let neighbours: Vec<_> = neighbours((x, y))
            .iter()
            .map(|neighbour| {
                format!(
                    "{}px",
                    if paths.iter().any(|(from, to)| {
                        (*from == (x, y) && *to == *neighbour)
                        || (*from == *neighbour && *to == (x, y))
                    }) {
                        0
                    } else {
                        BORDER_WIDTH
                    }
                )
            })
            .collect();
        let borders = neighbours.join(" ");
        html! {
            <div
            style={format!("border-width: {borders}")}
            class={classes!(
                "border-white", "size-full",
                ((x, y) == start).then_some("bg-green-500"),
                ((x, y) == goal).then_some("bg-red-500"),
            )}
            />
        }
    })
    })
    .collect::<VNode>();
    (
        Box::new(move |_username, password| password.contains(&solution)),
        html! {
            <div>
            <p>{"Password must contain the optimal solution to this maze, from green to red."}</p>
            <p>{"R is up, D is right, L is down, U is left."}</p>
            <div class="p-4 aspect-square bg-slate-600">
            <div class="grid grid-cols-[repeat(20,minmax(0,1fr))] size-full">
            {maze}
            </div>
            </div>
            </div>
        },
    )
    },
    (
        Box::new(
            |_username, password|
                password.contains(&Local::now().format("%-H:%M").to_string())
        ),
        "Password must contain the current time in the format HH:MM.".into(),
    ),
                (
                Box::new(
                    |_username, password|
                    *password == password.chars().rev().collect::<String>()
                ),
                "Password must be a palindrome.".into(),
            ),
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
