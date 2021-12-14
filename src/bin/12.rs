use std::collections::HashMap;

fn main() {
    let puzzle = Network::from_str(include_str!("../../input/12.txt"));
    println!("First solution: {}", puzzle.solve(Path::default(), false));
    println!("Second solution: {}", puzzle.solve(Path::default(), true));
}

type Cave = &'static str;

struct Network {
    connections: HashMap<Cave, Vec<Cave>>,
}

#[derive(Clone, Debug, Default)]
struct Path {
    caves: Vec<Cave>,
}

impl Network {
    // Recursively count the number of paths.
    fn solve(&self, current: Path, part2: bool) -> usize {
        if current.end() == "end" {
            return 1;
        }

        let mut n = 0;

        for &next in &self.connections[current.end()] {
            let can_visit = if part2 {
                current.can_visit_plus(next)
            } else {
                current.can_visit(next)
            };

            if can_visit && next != "start" {
                let mut next_path = current.clone();
                next_path.visit(next);
                n += self.solve(next_path, part2);
            }
        }
        n
    }

    fn from_str(s: &'static str) -> Self {
        let mut connections: HashMap<Cave, Vec<Cave>> = HashMap::new();
        for (from, to) in s.lines().filter_map(|l| l.trim().split_once('-')) {
            connections.entry(from).or_default().push(to);
            connections.entry(to).or_default().push(from);
        }

        Self { connections }
    }
}

impl Path {
    fn can_visit(&self, c: Cave) -> bool {
        if c.to_lowercase() == c {
            // Small caves may be visitied at most once
            !self.caves.iter().any(|&cc| c == cc)
        } else {
            // Big caves can be visited any number of times
            true
        }
    }
    // Allows a single small cave to be visited twice.
    fn can_visit_plus(&self, c: Cave) -> bool {
        if c.to_lowercase() == c {
            let already_visited = self.caves.iter().any(|&cc| c == cc);
            if already_visited {
                // We have already been to this small cave. Find out if we visited any single
                // cave twice already.
                let mut only_small_caves: Vec<Cave> = self
                    .caves
                    .clone()
                    .into_iter()
                    .filter(|&s| s.to_lowercase() == s)
                    .collect();
                only_small_caves.sort_unstable();
                let a = only_small_caves.len();
                only_small_caves.dedup();
                let b = only_small_caves.len();
                return a == b;
            }
        }
        true
    }
    fn visit(&mut self, c: Cave) {
        self.caves.push(c);
    }
    fn end(&self) -> Cave {
        self.caves.last().unwrap_or(&"start")
    }
}

#[cfg(test)]
mod test {
    use crate::{Network, Path};

    #[test]
    fn input10() {
        assert_eq!(
            Network::from_str(
                r"start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
        "
            )
            .solve(Path::default(), false),
            10
        );
    }

    #[test]
    fn input10plus() {
        assert_eq!(
            Network::from_str(
                r"start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
        "
            )
            .solve(Path::default(), true),
            36
        );
    }

    #[test]
    fn input19() {
        assert_eq!(
            Network::from_str(
                r"dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
        "
            )
            .solve(Path::default(), false),
            19
        );
    }

    #[test]
    fn input226() {
        assert_eq!(
            Network::from_str(
                r"fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
        "
            )
            .solve(Path::default(), false),
            226
        );
    }
}
