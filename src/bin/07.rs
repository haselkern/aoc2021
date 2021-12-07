use std::iter;

fn main() {
    println!("First solution: {}", find_fuel_min(|n| n));
    println!("Second solution: {}", find_fuel_min(|n| n * (n + 1) / 2));
}

fn find_fuel_min(cost: fn(i64) -> i64) -> i64 {
    let mut min = i64::MAX;
    let crabs = input();
    for i in 0..3000 {
        let fuel = crabs
            .iter()
            .zip(iter::repeat(i))
            .map(|(&a, b)| cost((a - b).abs()))
            .sum();
        min = min.min(fuel);
    }
    min
}

fn input() -> Vec<i64> {
    include_str!("../../input/07.txt")
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}
