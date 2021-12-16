use std::ops::{Index, IndexMut};

fn main() {
    let mut puzzle = input();
    println!("First solution: {}", puzzle.solve());
    let mut puzzle = input();
    puzzle.expand();
    println!("Second solution: {}", puzzle.solve());
}

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct Node {
    visited: bool,
    distance: u32,
    value: u32,
}

impl From<u32> for Node {
    fn from(v: u32) -> Self {
        Self {
            visited: false,
            distance: u32::MAX,
            value: v,
        }
    }
}

#[derive(Debug)]
struct Puzzle(Vec<Vec<Node>>);

impl Puzzle {
    /// Solve the puzzle with Dijsktra
    fn solve(&mut self) -> u32 {
        let mut stack: Vec<Position> = [(0, 0)].into_iter().collect();
        self[(0, 0)].distance = 0;

        while !stack.is_empty() {
            // Pop position with smallest distance, we will work on that node next.
            let mut smallest_idx = 0;
            let mut smallest_dist = u32::MAX;
            for (i, p) in stack.iter().enumerate() {
                if self[*p].distance < smallest_dist {
                    smallest_dist = self[*p].distance;
                    smallest_idx = i;
                }
            }

            let pos = stack.remove(smallest_idx);

            if self[pos].visited {
                continue;
            } else {
                self[pos].visited = true;
            }

            // Update neighbors and add them to the stack
            for next in self.neighbors(pos) {
                let new_dist = self[next].value + self[pos].distance;
                if new_dist < self[next].distance {
                    self[next].distance = new_dist;
                }
                stack.push(next);
            }
        }

        self.0.last().unwrap().last().unwrap().distance
    }

    /// Only returns unvisited neighbors
    fn neighbors(&self, p: Position) -> Vec<Position> {
        let mut result = Vec::new();
        if p.0 > 0 {
            result.push((p.0 - 1, p.1));
        }
        if p.1 > 0 {
            result.push((p.0, p.1 - 1));
        }
        if p.0 < self.0.len() - 1 {
            result.push((p.0 + 1, p.1));
        }
        if p.1 < self.0[0].len() - 1 {
            result.push((p.0, p.1 + 1));
        }
        result
            .into_iter()
            .filter(|p| !self.0[p.0][p.1].visited)
            .collect()
    }

    // Prepare the puzzle for part 2
    fn expand(&mut self) {
        let small_height = self.0.len();
        let small_width = self.0[0].len();
        let new_height = small_height * 5;
        let new_width = small_width * 5;

        let mut field = Vec::new();
        for row in 0..new_height {
            let mut row_values = Vec::new();
            for col in 0..new_width {
                let small_row = row % small_height;
                let small_col = col % small_width;
                let add = (row / small_height) + (col / small_width);
                let value = self[(small_row, small_col)].value;
                row_values.push((((value + add as u32) - 1) % 9) + 1);
            }
            field.push(row_values.into_iter().map(Node::from).collect());
        }

        self.0 = field;
    }
}

impl Index<Position> for Puzzle {
    type Output = Node;

    fn index(&self, index: Position) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl IndexMut<Position> for Puzzle {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

fn input() -> Puzzle {
    Puzzle(
        include_str!("../../input/15.txt")
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .map(Node::from)
                    .collect()
            })
            .collect(),
    )
}
