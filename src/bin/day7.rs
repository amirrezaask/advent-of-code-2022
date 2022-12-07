use anyhow::Result;
use std::{collections::HashMap, str::FromStr};

fn is_numeric(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

#[derive(Debug)]
enum UserCommandType {
    Cd,
    Ls,
}

#[derive(Debug)]
enum Output {
    Dir { name: String },

    File { name: String, size: u64 },
}
#[derive(Debug)]
enum Line {
    UserCommand {
        _type: UserCommandType,
        input: String,
    },
    Dir {
        name: String,
    },

    File {
        name: String,
        size: u64,
    },
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "" {
            return Err(anyhow::format_err!("empty line"));
        }
        if s.chars().nth(0).unwrap() == '$' {
            let mut chars = s.chars();
            chars.next();
            chars.next();
            let command_first_char = chars.next().unwrap();
            if command_first_char == 'c' {
                chars.next();
                chars.next();
                let dir_name: String = chars.collect();
                return Ok(Line::UserCommand {
                    _type: UserCommandType::Cd,
                    input: dir_name,
                });
            } else if command_first_char == 'l' {
                return Ok(Line::UserCommand {
                    _type: UserCommandType::Ls,
                    input: "".to_string(),
                });
            }
        } else {
            let (left, right) = s.split_once(" ").unwrap();
            match left.parse::<u64>() {
                Ok(size) => {
                    return Ok(Line::File {
                        size,
                        name: right.to_string(),
                    })
                }
                Err(_) => {
                    return Ok(Line::Dir {
                        name: right.to_string(),
                    })
                }
            }
        }
        panic!()
    }
}

fn add_file_size(sizes: &mut HashMap<Vec<String>, u64>, mut current: Vec<String>, file_size: u64) {
    loop {
        match sizes.get_mut(&current) {
            Some(dir) => {
                *dir += file_size;
            }
            None => {
                sizes.insert(current.clone(), file_size);
            }
        }

        if current.len() == 0 {
            break;
        }
        current.pop();
    }
}

fn main() -> Result<()> {
    let input = include_str!("input7.prod");

    let lines = input
        .split("\n")
        .filter_map(|l| l.parse::<Line>().ok())
        .collect::<Vec<Line>>();

    let mut sizes: HashMap<Vec<String>, u64> = HashMap::new();
    let mut current: Vec<String> = vec![];

    for line in lines {
        match line {
            Line::UserCommand {
                _type: UserCommandType::Cd,
                input,
            } => {
                if input == ".." {
                    current.pop();
                } else {
                    current.push(input);
                }
            }
            Line::UserCommand {
                _type: UserCommandType::Ls,
                ..
            } => {}

            Line::Dir { .. } => {}
            Line::File { size, .. } => {
                add_file_size(&mut sizes, current.clone(), size);
            }
        }
    }
    let mut sum = 0;
    for (k,v) in &sizes {
        if v <= &100000 {
            sum += v;
        }
    }

    println!("sizes: {:?}", sizes);
    println!("sum: {}", sum);

    return Ok(());
}
