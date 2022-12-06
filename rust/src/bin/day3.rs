use std::collections::HashSet;

use anyhow::Result;


fn find_mutual_chars(s1: &str, s2: &str) -> HashSet<char> {
    let mut hs: HashSet<char> = HashSet::new();
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                hs.insert(c1);
            }
        }
    }

    return hs;
}

fn char_priority(c: char) -> usize {
    let c = c as u8;
    if c >= 97 && c <= 122 {
        return (c - 97 + 1).into();
    } else if c  >= 65 && c <= 90 {
        return (c - 65 + 27).into();
    }

    return 0;

}

fn main() -> Result<()> {
    let result = include_str!("input3.prod")
        .split("\n")
        .flat_map(|line| {
            let (first, second) = line.split_at(line.len()/2);
            let mutual = find_mutual_chars(first, second);
            mutual.into_iter().map(|m| {
                char_priority(m)
            })
        }).sum::<usize>();
    println!("part 1 : {}", result);
    return Ok(());
}
