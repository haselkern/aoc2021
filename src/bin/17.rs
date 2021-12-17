fn main() {
    let (max_y, count) = solve();
    println!("First solution: {:?}", max_y);
    println!("Second solution: {:?}", count);
}

fn solve() -> (i32, i32) {
    let mut max_y = 0;
    let mut count = 0;

    for vx in 1..150 {
        for vy in -1000..1000 {
            if let Some(y) = check_trajectory(vx, vy) {
                count += 1;
                max_y = max_y.max(y);
            }
        }
    }

    (max_y, count)
}

/// Returns the largest y if it hits the target.
fn check_trajectory(mut vx: i32, mut vy: i32) -> Option<i32> {
    let mut pos = Position::default();
    let mut max_y = 0;
    let target = input();

    loop {
        pos.x += vx;
        pos.y += vy;
        vx -= vx.signum();
        vy -= 1;
        max_y = max_y.max(pos.y);

        if target.contains(pos) {
            return Some(max_y);
        }
        if pos.y < target.min.y {
            // We are below the target
            return None;
        }
        if pos.x > target.max.x {
            // We are too far right
            return None;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Target {
    min: Position,
    max: Position,
}

impl Target {
    fn contains(&self, p: Position) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}

fn input() -> Target {
    let file = include_str!("../../input/17.txt");
    let (_, coords) = file.split_once(": ").unwrap();
    let (x, y) = coords.split_once(", ").unwrap();
    let x = &x[2..];
    let y = &y[2..];

    let (x_min, x_max) = x.split_once("..").unwrap();
    let (y_min, y_max) = y.split_once("..").unwrap();

    Target {
        min: Position {
            x: x_min.parse().unwrap(),
            y: y_min.parse().unwrap(),
        },
        max: Position {
            x: x_max.parse().unwrap(),
            y: y_max.parse().unwrap(),
        },
    }
}
