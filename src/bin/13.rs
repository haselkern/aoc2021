use std::{collections::HashSet, fmt::Display};

fn main() {
    let (mut paper, mut instructions) = input();

    paper.apply(instructions.next().unwrap());
    println!("First solution: {}", paper.0.len());

    for i in instructions {
        paper.apply(i);
    }
    println!("Second solution:\n{}", paper);
}

struct Paper(HashSet<(usize, usize)>);

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (width, height) = self.size();
        for y in 0..=height {
            for x in 0..=width {
                if self.0.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Paper {
    fn apply(&mut self, instruction: Instruction) {
        self.0 = self
            .0
            .iter()
            .copied()
            .map(|(x, y)| match instruction {
                Instruction::X(n) => {
                    if x > n {
                        (x - 2 * (x - n), y)
                    } else {
                        (x, y)
                    }
                }
                Instruction::Y(n) => {
                    if y > n {
                        (x, y - 2 * (y - n))
                    } else {
                        (x, y)
                    }
                }
            })
            .collect()
    }

    /// Returns the highest x and the highest y
    fn size(&self) -> (usize, usize) {
        self.0
            .iter()
            .fold((0, 0), |(x1, y1), (x2, y2)| (x1.max(*x2), y1.max(*y2)))
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    X(usize),
    Y(usize),
}

fn input() -> (Paper, impl Iterator<Item = Instruction>) {
    let file = include_str!("../../input/13.txt");

    let paper = Paper(
        file.lines()
            .filter(|l| l.contains(','))
            .map(|l| l.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect(),
    );

    let instructions =
        file.lines()
            .filter(|l| l.contains('='))
            .map(|l| match l.split_once('=').unwrap() {
                ("fold along x", n) => Instruction::X(n.parse().unwrap()),
                ("fold along y", n) => Instruction::Y(n.parse().unwrap()),
                x => panic!("unknown instruction: {:?}", x),
            });

    (paper, instructions)
}
