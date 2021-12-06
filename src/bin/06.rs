use std::collections::VecDeque;

type School = VecDeque<u64>;

fn main() {
    let mut fish = input();
    for _ in 0..80 {
        step(&mut fish);
    }
    println!("First solution: {}", sum(&fish));

    let mut fish = input();
    for _ in 0..256 {
        step(&mut fish);
    }
    println!("Second solution: {}", sum(&fish));
}

fn step(fish: &mut School) {
    let front = fish.pop_front().unwrap();
    fish.push_back(front);
    fish[6] += front;
}

fn sum(fish: &School) -> u64 {
    fish.iter().sum()
}

fn input() -> School {
    let init: School = vec![0; 9].into();
    include_str!("../../input/06.txt")
        .split(',')
        .map(|n| n.parse().unwrap())
        .fold(init, |mut acc, n| {
            acc[n] += 1;
            acc
        })
}
