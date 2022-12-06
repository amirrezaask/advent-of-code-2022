use anyhow::Result;

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn from_op(s: &str) -> Self {
        match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn from_me(s: &str) -> Self {
        match s {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

struct Part1Strategy {
    op_move: Move,
    my_move: Move,
}

enum OutCome {
    Win,
    Loss,
    Draw,
}
fn move_score(m: &Move) -> usize {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

struct Part2Strategy {
    op_move: Move,
    result: OutCome,
}

impl Part2Strategy {
    fn score(&self) -> usize {
        match self {
            Part2Strategy {
                op_move: Move::Rock,
                result: OutCome::Win,
            } => move_score(&Move::Paper) + WIN_SCORE,
            Part2Strategy {
                op_move: Move::Paper,
                result: OutCome::Win,
            } => move_score(&Move::Scissors) + WIN_SCORE,
            Part2Strategy {
                op_move: Move::Scissors,
                result: OutCome::Win,
            } => move_score(&Move::Rock) + WIN_SCORE,

            Part2Strategy {
                op_move: Move::Rock,
                result: OutCome::Loss,
            } => move_score(&Move::Scissors) + LOSS_SCORE,
            Part2Strategy {
                op_move: Move::Paper,
                result: OutCome::Loss,
            } => move_score(&Move::Rock) + LOSS_SCORE,
            Part2Strategy {
                op_move: Move::Scissors,
                result: OutCome::Loss,
            } => move_score(&Move::Paper) + LOSS_SCORE,

            Part2Strategy {
                op_move: Move::Rock,
                result: OutCome::Draw,
            } => move_score(&Move::Rock) + DRAW_SCORE,

            Part2Strategy {
                op_move: Move::Paper,
                result: OutCome::Draw,
            } => move_score(&Move::Paper) + DRAW_SCORE,
            Part2Strategy {
                op_move: Move::Scissors,
                result: OutCome::Draw,
            } => move_score(&Move::Scissors) + DRAW_SCORE,
        }
    }
}

impl From<&str> for OutCome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

const WIN_SCORE: usize = 6;
const LOSS_SCORE: usize = 0;
const DRAW_SCORE: usize = 3;

impl Part1Strategy {
    fn score(&self) -> usize {
        match self {
            Part1Strategy {
                op_move: Move::Rock,
                my_move: Move::Scissors,
            } => LOSS_SCORE + move_score(&self.my_move),

            Part1Strategy {
                op_move: Move::Rock,
                my_move: Move::Paper,
            } => WIN_SCORE + move_score(&self.my_move),

            Part1Strategy {
                op_move: Move::Rock,
                my_move: Move::Rock,
            } => DRAW_SCORE + move_score(&self.my_move),

            Part1Strategy {
                op_move: Move::Paper,
                my_move: Move::Scissors,
            } => WIN_SCORE + move_score(&self.my_move),

            Part1Strategy {
                op_move: Move::Paper,
                my_move: Move::Paper,
            } => DRAW_SCORE + move_score(&self.my_move),

            Part1Strategy {
                op_move: Move::Paper,
                my_move: Move::Rock,
            } => LOSS_SCORE + move_score(&self.my_move),

            Part1Strategy {
                op_move: Move::Scissors,
                my_move: Move::Scissors,
            } => DRAW_SCORE + move_score(&self.my_move),

            Part1Strategy {
                op_move: Move::Scissors,
                my_move: Move::Paper,
            } => LOSS_SCORE + move_score(&self.my_move),

            Part1Strategy {
                op_move: Move::Scissors,
                my_move: Move::Rock,
            } => WIN_SCORE + move_score(&self.my_move),
        }
    }
}

fn main() -> Result<()> {
    let result1 = include_str!("input2.prod")
        .split("\n")
        .map(|st| {
            if st == "" {
                return 0;
            }
            let splitted: Vec<&str> = st.split(" ").collect();
            let s1 = Part1Strategy {
                op_move: Move::from_op(splitted[0]),
                my_move: Move::from_me(splitted[1]),
            };

            let s2 = Part2Strategy {
                op_move: Move::from_op(splitted[0]),
                result: splitted[1].into(),
            };
            s1.score()
        })
        .sum::<usize>();
    let result2 = include_str!("input2.prod")
        .split("\n")
        .map(|st| {
            if st == "" {
                return 0;
            }
            let splitted: Vec<&str> = st.split(" ").collect();
            let s2 = Part2Strategy {
                op_move: Move::from_op(splitted[0]),
                result: splitted[1].into(),
            };
            s2.score()
        })
        .sum::<usize>();

    println!("part 1: {}", result1);
    println!("part 2: {}", result2);
    return Ok(());
}
