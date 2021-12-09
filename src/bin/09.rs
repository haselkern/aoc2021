fn main() {
    println!("First solution: {}", input().risk());
    println!("Second solution: {}", input().part2());
}

struct Field(Vec<Vec<u32>>);

impl Field {
    fn risk(&self) -> u32 {
        let mut risk = 0;
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                let current = self.0[row][col];
                if self.neighbor_values(row, col).iter().all(|&n| n > current) {
                    risk += current + 1;
                }
            }
        }
        risk
    }

    fn neighbor_values(&self, row: usize, col: usize) -> Vec<u32> {
        self.neighbor_positions(row, col)
            .into_iter()
            .map(|(row, col)| self.0[row][col])
            .collect()
    }

    fn neighbor_positions(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if row > 0 {
            result.push((row - 1, col));
        }
        if row < self.0.len() - 1 {
            result.push((row + 1, col));
        }
        if col > 0 {
            result.push((row, col - 1));
        }
        if col < self.0[0].len() - 1 {
            result.push((row, col + 1));
        }
        result
    }

    fn part2(&mut self) -> u32 {
        let mut basin_sizes = Vec::new();

        loop {
            let mut fill_start = None;
            'search: for row in 0..self.0.len() {
                for col in 0..self.0[0].len() {
                    if self.0[row][col] < 9 {
                        fill_start = Some((row, col));
                        break 'search;
                    }
                }
            }

            let start = match fill_start {
                Some(f) => f,
                None => break,
            };

            let mut fill_stack = vec![start];
            let mut size = 0;
            while let Some((row, col)) = fill_stack.pop() {
                if self.0[row][col] >= 9 {
                    continue;
                }
                size += 1;
                self.0[row][col] = 9; // Mark field as processed
                for (nr, nc) in self.neighbor_positions(row, col) {
                    fill_stack.push((nr, nc));
                }
            }
            basin_sizes.push(size);
        }

        basin_sizes.sort_unstable();
        basin_sizes.into_iter().rev().take(3).product()
    }
}

fn input() -> Field {
    Field(
        include_str!("../../input/09.txt")
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect(),
    )
}
