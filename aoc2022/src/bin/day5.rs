use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, Clone)]
struct Stack(Vec<char>);
impl Stack {
    fn pop(&mut self) -> char {
        let elem = self.0[0];
        self.0.remove(0);
        return elem;
    }

    fn push(&mut self, elem: char) {
        self.0.insert(0, elem);
    }
}

fn parse_stack_line(s: &str) -> Vec<char> {
    return s
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| {
            if chunk[0] == ' ' && chunk[1] == ' ' && chunk[2] == ' ' {
                return 0 as char;
            } else {
                return chunk[1];
            }
        })
        .collect::<Vec<char>>();
}
#[derive(Debug)]
struct Command {
    count: usize,
    from: usize,
    to: usize,
}

impl Command {
    fn execute1(&self, s: &mut HashMap<usize, Stack>) {
        let from = s.get_mut(&self.from).unwrap();
        let mut elems: Vec<char> = Vec::new();
        for i in 0..self.count {
            let elem = from.pop();
            elems.push(elem);
        }

        let to = s.get_mut(&self.to).unwrap();
        for elem in elems {
            to.push(elem);
        }
    }

    fn execute2(&self, s: &mut HashMap<usize, Stack>) {
        let from = s.get_mut(&self.from).unwrap();

        let mut elems = from.0.iter().take(self.count).map(|c| *c).collect::<Vec<char>>();
        for _ in 0..self.count {
            from.0.remove(0);

        }

        let to = s.get_mut(&self.to).unwrap();
        elems.append(&mut to.0);
        to.0 = elems;
    }

}

fn parse_command(s: &str) -> Command {
    let splitted = s.split(" ").collect::<Vec<&str>>();

    Command {
        count: splitted[1].parse::<usize>().unwrap(),
        from: splitted[3].parse::<usize>().unwrap(),
        to: splitted[5].parse::<usize>().unwrap(),
    }
}

fn main() -> Result<()> {
    let mut stacks1: HashMap<usize, Stack> = HashMap::new();
    let mut input = include_str!("input5.prod").split("\n\n");

    let init: Vec<&str> = input.nth(0).unwrap().split("\n").collect();
    // remove last line
    let len_init = init.len();

    init.into_iter()
        .take(len_init - 1)
        .map(|l| parse_stack_line(l))
        .for_each(|l| {
            l.iter().enumerate().for_each(|(idx, c)| {
                if *c == '\0' {
                    return;
                }
                match stacks1.get_mut(&(idx + 1)) {
                    Some(stack) => stack.0.push(*c),
                    None => {
                        stacks1.insert(idx + 1, Stack(vec![*c]));
                    }
                }
            })
        });

    let commands = input
        .nth(0)
        .unwrap()
        .split("\n")
        .filter(|l| l != &"")
        .map(|l| parse_command(l))
        .collect::<Vec<Command>>();

    let mut stacks2 = stacks1.clone();
    for command in commands {
        // println!("command: {:?}", command);
        command.execute1(&mut stacks1);
        command.execute2(&mut stacks2);
        // println!("stack: {:?}", stacks);
    }

    println!("part1 : ");
    let mut keys = stacks1.keys().collect::<Vec<&usize>>();
    keys.sort();
    for key in keys {
        println!("{:?}", stacks1.get(key).unwrap())
    }
    let mut keys = stacks1.keys().collect::<Vec<&usize>>();

    keys.sort();

    println!("part2 : ");
    for key in keys {
        println!("{:?}", stacks2.get(key).unwrap())
    }


    return Ok(());
}
