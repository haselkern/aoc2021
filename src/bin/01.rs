const INPUT: &str = include_str!("../../input/01a.txt");

fn main() {
    let nums = input();
    let count = count_increases(nums);
    println!("First solution: {}", count);

    let input: Vec<i32> = input().collect();
    let nums = input.windows(3).map(|w| w.iter().sum());
    let count = count_increases(nums);
    println!("Second solution: {}", count);
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
    INPUT.lines().map(|l| l.parse::<i32>().unwrap())
}
