fn main() {
    println!("First solution: {}", count_increases(input()));

    let input: Vec<i32> = input().collect();
    let nums = input.windows(3).map(|w| w.iter().sum());
    println!("Second solution: {}", count_increases(nums));
}

fn count_increases(it: impl Iterator<Item = i32>) -> usize {
    let mut count = 0;
    let mut prev = None;

    for n in it {
        if let Some(prev) = prev {
            if n > prev {
                count += 1;
            }
        }
        prev = Some(n);
    }

    count
}

fn input() -> impl Iterator<Item = i32> {
    include_str!("../../input/01a.txt")
        .lines()
        .map(|l| l.parse().unwrap())
}
