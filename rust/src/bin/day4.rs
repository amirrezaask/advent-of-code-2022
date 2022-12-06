use anyhow::Result;

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, r: &Self) -> bool {
        return r.start >= self.start && r.end <= self.end;
    }

    fn overlap(&self, r: &Self) -> bool {
        return (r.start <= self.end && r.end >= self.start)
            || (self.start <= r.end && self.end >= r.start);
    }
}

fn parse_range(s: &str) -> Range {
    let splitted = s.split("-").collect::<Vec<&str>>();

    return Range {
        start: splitted[0].parse::<usize>().unwrap(),
        end: splitted[1].parse::<usize>().unwrap(),
    };
}

fn main() -> Result<()> {
    let parsed = include_str!("input4.prod")
        .split("\n")
        .filter(|line| line != &"")
        .map(|line| {
            let splitted: Vec<&str> = line.split(",").collect();
            (parse_range(splitted[0]), parse_range(splitted[1]))
        });

    let result1 = parsed.clone()
        .filter(|(first, second)| first.contains(&second) || second.contains(&first))
        .count();

    let result2 = parsed.filter(|(first, second)| first.overlap(&second) || second.overlap(&first)).count();

    println!("part 1: {:?}", result1);
    println!("part 2: {:?}", result2);

    return Ok(());
}
