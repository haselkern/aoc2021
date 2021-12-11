fn main() {
    let (flashes, sync) = solve();
    println!("First solution: {}", flashes);
    println!("Second solution: {}", sync);
}

fn solve() -> (usize, usize) {
    let mut field = input();
    let mut flashes = 0;
    let mut i = 0;

    loop {
        i += 1;

        // Add one to each octopus
        field.apply(|o| {
            o.energy += 1;
        });

        // Do the flashing
        let mut changed = true;
        while changed {
            changed = false;

            for row in 0..10 {
                for col in 0..10 {
                    let current = &mut field.0[row][col];
                    if current.energy > 9 && !current.did_flash {
                        current.did_flash = true;
                        changed = true;
                        if i <= 100 {
                            flashes += 1;
                        }
                        for (r, c) in neighbors(row, col) {
                            field.0[r][c].energy += 1;
                        }
                    }
                }
            }
        }

        // Have all octopi flashed?
        if field.0.iter().flat_map(|r| r.iter()).all(|o| o.did_flash) {
            return (flashes, i);
        }

        // Reset octopi > 9
        field.apply(|o| {
            if o.energy > 9 {
                o.energy = 0;
                o.did_flash = false;
            }
        });
    }
}

fn neighbors(row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    let mut rs = vec![row];
    let mut cs = vec![col];

    if row > 0 {
        rs.push(row - 1);
    }
    if row < 9 {
        rs.push(row + 1);
    }
    if col > 0 {
        cs.push(col - 1);
    }
    if col < 9 {
        cs.push(col + 1);
    }

    for r in rs {
        for &c in &cs {
            if r == row && c == col {
                continue;
            }
            neighbors.push((r, c));
        }
    }

    neighbors
}

struct Field(Vec<Vec<Octopus>>);

impl Field {
    /// Do something with every octopus
    fn apply<F>(&mut self, mut op: F)
    where
        F: FnMut(&mut Octopus),
    {
        for row in 0..10 {
            for col in 0..10 {
                op(&mut self.0[row][col])
            }
        }
    }
}

struct Octopus {
    energy: u32,
    did_flash: bool,
}

fn input() -> Field {
    Field(
        include_str!("../../input/11.txt")
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .map(|n| Octopus {
                        energy: n,
                        did_flash: false,
                    })
                    .collect()
            })
            .collect(),
    )
}
