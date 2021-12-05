use std::{cmp::Ordering, fmt::Display, iter};

fn main() {
    let mut f = Field::new(1000);
    for line in input().filter(|line| line.col1 == line.col2 || line.row1 == line.row2) {
        f.apply(line);
    }
    println!("First solution: {}", f.overlaps());

    let mut f = Field::new(1000);
    for line in input() {
        f.apply(line);
    }
    println!("Second solution: {}", f.overlaps());
}

struct Field {
    /// row-major encoding of the field
    count: Vec<Vec<u32>>,
}

impl Field {
    fn new(size: usize) -> Self {
        Self {
            count: vec![vec![0; size]; size],
        }
    }

    fn apply(&mut self, line: Line) {
        for (row, col) in line.points() {
            self.count[row][col] += 1;
        }
    }

    fn overlaps(&self) -> usize {
        self.count
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&n| n >= 2)
            .count()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.count.len() {
            for col in 0..self.count.len() {
                let c = match self.count[row][col] {
                    0 => ".".to_string(),
                    n if n <= 9 => n.to_string(),
                    _ => "#".to_string(),
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Line {
    col1: usize,
    row1: usize,
    col2: usize,
    row2: usize,
}

impl Line {
    fn points(&self) -> impl Iterator<Item = (usize, usize)> {
        self.row().zip(self.col()).take(self.len())
    }
    fn col(&self) -> Box<dyn Iterator<Item = usize>> {
        match self.col1.cmp(&self.col2) {
            Ordering::Less => Box::new(self.col1..=self.col2),
            Ordering::Equal => Box::new(iter::repeat(self.col1)),
            Ordering::Greater => Box::new((self.col2..=self.col1).rev()),
        }
    }
    fn row(&self) -> Box<dyn Iterator<Item = usize>> {
        match self.row1.cmp(&self.row2) {
            Ordering::Less => Box::new(self.row1..=self.row2),
            Ordering::Equal => Box::new(iter::repeat(self.row1)),
            Ordering::Greater => Box::new((self.row2..=self.row1).rev()),
        }
    }
    fn len(&self) -> usize {
        let row = if self.row1 < self.row2 {
            self.row2 - self.row1
        } else {
            self.row1 - self.row2
        };
        let col = if self.col1 < self.col2 {
            self.col2 - self.col1
        } else {
            self.col1 - self.col2
        };
        col.max(row) + 1
    }
}

fn input() -> impl Iterator<Item = Line> {
    include_str!("../../input/05.txt")
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(start, end)| {
            let (x1, y1) = start.split_once(",").unwrap();
            let (x2, y2) = end.split_once(",").unwrap();
            Line {
                col1: x1.parse().unwrap(),
                row1: y1.parse().unwrap(),
                col2: x2.parse().unwrap(),
                row2: y2.parse().unwrap(),
            }
        })
}
