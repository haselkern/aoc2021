use std::fmt::Display;

fn main() {
    println!("First score: {}\n", part1());
    println!("Second score: {}", part2());
}

fn part1() -> u32 {
    let (mut boards, rng) = input();

    for n in rng {
        for board in &mut boards {
            board.mark(n);
            if board.wins() {
                println!("{}", board);
                return board.score(n);
            }
        }
    }

    0
}

fn part2() -> u32 {
    let (mut boards, mut rng) = input();

    let mut last_n = 0;
    while boards.len() > 1 {
        last_n = rng.next().unwrap();
        for board in &mut boards {
            board.mark(last_n);
        }
        boards.retain(|b| !b.wins());
    }

    // Only one board remains, play until it wins
    let board = &mut boards[0];
    while !board.wins() {
        last_n = rng.next().unwrap();
        board.mark(last_n);
    }
    println!("{}", board);

    board.score(last_n)
}

#[derive(Debug)]
struct Board {
    nums: [u32; 25],
    marked: [bool; 25],
}

impl Board {
    fn mark(&mut self, n: u32) {
        for i in 0..25 {
            if self.nums[i] == n {
                self.marked[i] = true;
            }
        }
    }

    fn score(&self, last_n: u32) -> u32 {
        let sum: u32 = self
            .nums
            .iter()
            .zip(self.marked.iter())
            .filter(|(_, &marked)| !marked)
            .map(|(n, _)| n)
            .sum();
        sum * last_n
    }

    fn wins(&self) -> bool {
        // Check rows
        for row in 0..5 {
            let mut win = true;
            for col in 0..5 {
                if !self.marked[row * 5 + col] {
                    win = false;
                    break;
                }
            }
            if win {
                return true;
            }
        }
        // Check cols
        for col in 0..5 {
            let mut win = true;
            for row in 0..5 {
                if !self.marked[row * 5 + col] {
                    win = false;
                    break;
                }
            }
            if win {
                return true;
            }
        }
        false
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
                let idx = row * 5 + col;
                write!(
                    f,
                    "{}{:02} ",
                    if self.marked[idx] { "*" } else { " " },
                    self.nums[idx]
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

// Return a list of boards and an rng
fn input() -> (Vec<Board>, impl Iterator<Item = u32>) {
    let mut lines = include_str!("../../input/04.txt").lines();

    let rng = lines.next().unwrap().split(',').map(|n| n.parse().unwrap());

    let lines: Vec<_> = lines.collect();

    let mut boards = Vec::new();

    // Chunks have one empty line and five board lines
    for board in lines.chunks_exact(6) {
        let b: Vec<u32> = board
            .iter()
            .flat_map(|l| l.split_ascii_whitespace())
            .map(|n| n.parse().unwrap())
            .collect();
        boards.push(Board {
            nums: b.try_into().unwrap(),
            marked: [false; 25],
        })
    }

    (boards, rng)
}
