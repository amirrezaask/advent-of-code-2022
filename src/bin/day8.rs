fn is_visible(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> bool {
    // println!("{} ({}, {})", grid[pos.0][pos.1], pos.0, pos.1);
    let height = grid[pos.0][pos.1];
    let (left, right) = grid[pos.0].split_at(pos.1);
    let left = left.to_vec();
    let right = right[1..].to_vec();

    let column = grid.iter().map(|row| row[pos.1]).collect::<Vec<u32>>();

    let (up, down) = column.split_at(pos.0);
    let up = up.to_vec();
    let down = down[1..].to_vec();

    // println!("up: {:?}", up);
    // println!("down: {:?}", down);
    // println!("left: {:?}", left);
    // println!("right: {:?}", right);

    return (left.iter().all(|h| h < &height)
        || right.iter().all(|h| h < &height)
        || up.iter().all(|h| h < &height)
        || down.iter().all(|h| h < &height));
}

fn main() {
    let input = include_str!("input8.prod");

    let grid = input
        .split("\n")
        .filter(|l| l != &"")
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let mut count = 0;

    // for row in &grid {
    //     println!("{:?}", row);
    // }
    //
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if is_visible(&grid, (row, col)) {
                // println!("({}, {})", row, col);
                count += 1;
            }
        }
    }
    // 2 2
    // 3 1

    println!("part 1: {}", count);
}
