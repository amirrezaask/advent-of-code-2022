fn scenic_score(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> u32 {
    let height = grid[pos.0][pos.1];
    let (left, right) = grid[pos.0].split_at(pos.1);
    let mut left = left.to_vec();
    let right = right[1..].to_vec();

    let column = grid.iter().map(|row| row[pos.1]).collect::<Vec<u32>>();

    let (up, down) = column.split_at(pos.0);
    let mut up = up.to_vec();
    let down = down[1..].to_vec();
    let mut scenic: Vec<u32> = vec![];


    up.reverse();
    left.reverse();

    // println!("height: {}", height);
    for dir in &[left, right, up, down] {
        // println!("dir: {:?}", dir);
        let mut count: u32 = 0;
        for t in dir {
            // println!("t: {}", t);
            if t < &height {
                count += 1;
            }
            if t >= &height {
                count += 1;
                break;
            }
        }

        scenic.push(count);
    }
    let score = scenic.clone().into_iter().reduce(|acc, item| (acc*item)).unwrap();
    // println!("({}, {}) scenic: {:?} score {}", pos.0, pos.1, scenic, score);

    return score;
}

fn is_visible(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> bool {
    let height = grid[pos.0][pos.1];
    let (left, right) = grid[pos.0].split_at(pos.1);
    let left = left.to_vec();
    let right = right[1..].to_vec();

    let column = grid.iter().map(|row| row[pos.1]).collect::<Vec<u32>>();

    let (up, down) = column.split_at(pos.0);
    let up = up.to_vec();
    let down = down[1..].to_vec();

    return left.iter().all(|h| h < &height)
        || right.iter().all(|h| h < &height)
        || up.iter().all(|h| h < &height)
        || down.iter().all(|h| h < &height);
}

fn main() {
    let input = include_str!("input8.prod");

    let grid = input
        .split("\n")
        .filter(|l| l != &"")
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    // scenic_score(&grid, (3,2));

    let mut count = 0;

    let mut highest_scenic = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if is_visible(&grid, (row, col)) {
                // println!("({}, {})", row, col);
                count += 1;
            }
            let scenic = scenic_score(&grid, (row, col));
            // println!("{}, {} scenic: {}", row, col, scenic);
            if scenic > highest_scenic {
                highest_scenic = scenic;
            }
        }
    }

    println!("part 1: {}", count);
    println!("part 2: {}", highest_scenic);
}
