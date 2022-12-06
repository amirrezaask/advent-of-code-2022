use anyhow::Result;

fn main() -> Result<()> {
    let mut result: Vec<usize> = include_str!("input1.prod")
        .split("\n\n")
        .map(|g| g.split("\n").flat_map(|num| num.parse::<usize>()).sum())
        .collect();

    result.sort_by(|a, b| b.cmp(a));
    println!("part 1: {:?}", result[0]);

    println!("part 2: {:?}", result.iter().take(3).sum::<usize>());
    return Ok(());
}
