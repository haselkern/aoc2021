use std::{collections::HashMap, hash::Hash};

fn main() {
    let mut puzzle = input();
    for _ in 0..10 {
        puzzle.step();
    }
    println!("First solution: {}", puzzle.score());
    for _ in 0..30 {
        puzzle.step();
    }
    println!("Second solution: {}", puzzle.score());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Digram(char, char);

#[derive(Debug)]
struct Puzzle {
    /// polymer is a count of all digrams in it.
    polymer: HashMap<Digram, usize>,
    /// start and end need to be kept track of, since all other chars are counted doubly,
    /// due to the nature of counting the digrams.
    start: char,
    end: char,
    instructions: Vec<Instruction>,
}

/// ab -> to
#[derive(Debug, Clone, Copy)]
struct Instruction {
    from: Digram,
    to: char,
}

impl Puzzle {
    fn step(&mut self) {
        let mut new_polymer = HashMap::new();
        for (&old_digram, &count) in &self.polymer {
            if let Some(i) = self.find_instruction(old_digram) {
                for new_digram in i.output() {
                    let counter = new_polymer.entry(new_digram).or_default();
                    *counter += count;
                }
            } else {
                let counter = new_polymer.entry(old_digram).or_default();
                *counter += count;
            }
        }
        self.polymer = new_polymer;
    }

    fn find_instruction(&self, d: Digram) -> Option<Instruction> {
        self.instructions.iter().copied().find(|i| i.from == d)
    }

    fn score(&self) -> usize {
        // All chars were counted doubly, due to the nature of the digrams, except start and end.
        // Add one for those manually here.
        let mut char_count: HashMap<char, usize> =
            [(self.start, 1), (self.end, 1)].into_iter().collect();

        for (&d, &count) in &self.polymer {
            for c in [d.0, d.1] {
                let counter = char_count.entry(c).or_default();
                *counter += count;
            }
        }

        let mut doubly_counted: Vec<usize> = char_count.values().copied().collect();
        doubly_counted.sort_unstable();

        (doubly_counted.last().unwrap() - doubly_counted.first().unwrap()) / 2
    }
}

impl Instruction {
    fn output(&self) -> [Digram; 2] {
        [Digram(self.from.0, self.to), Digram(self.to, self.from.1)]
    }
}

fn input() -> Puzzle {
    let file = include_str!("../../input/14.txt");

    let polymer_chars: Vec<char> = file.lines().next().unwrap().trim().chars().collect();
    let polymer = polymer_chars
        .windows(2)
        .into_iter()
        .map(|w| Digram(w[0], w[1]))
        .fold(HashMap::new(), |mut acc, d| {
            let count = acc.entry(d).or_default();
            *count += 1;
            acc
        });

    let instructions = file
        .lines()
        .filter(|l| l.contains("->"))
        .filter_map(|l| l.split_once(" -> "))
        .map(|(from, to)| {
            let a = from.chars().next().unwrap();
            let b = from.chars().nth(1).unwrap();
            Instruction {
                from: Digram(a, b),
                to: to.chars().next().unwrap(),
            }
        })
        .collect();

    Puzzle {
        polymer,
        instructions,
        start: polymer_chars.first().copied().unwrap(),
        end: polymer_chars.last().copied().unwrap(),
    }
}
