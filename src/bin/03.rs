fn main() {
    let gamma_bits = gamma(&input());
    let gamma = to_dec(&gamma_bits);
    let epsilon = to_dec(&invert(&gamma_bits));
    println!(
        "First solution: gamma={}, epsilon={}, product={}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let oxygen = to_dec(&life_support(input(), false));
    let co2 = to_dec(&life_support(input(), true));
    println!(
        "Second solution: oxygen={}, co2={}, product={}",
        oxygen,
        co2,
        oxygen * co2
    );
}

type Binary = Vec<i32>;

fn life_support(mut it: Vec<Binary>, co2: bool) -> Binary {
    let mut i = 0;
    while it.len() > 1 {
        let gamma = gamma(&it);
        it = it
            .into_iter()
            .filter(|b| b[i] == gamma[i] ^ co2 as i32)
            .collect();
        i += 1;
    }
    it.remove(0)
}

fn to_dec(b: &Binary) -> i32 {
    let mut acc = 0;

    for (i, n) in b.iter().rev().copied().enumerate() {
        acc |= n << i;
    }

    acc
}

fn invert(b: &Binary) -> Binary {
    b.iter().map(|&n| n ^ 1).collect()
}

/// gamma returns a pattern of the most common bits for each position.
fn gamma(it: &Vec<Binary>) -> Binary {
    let mut iter = it.clone().into_iter();
    let mut acc = iter.next().unwrap();

    for n in iter {
        for i in 0..acc.len() {
            acc[i] += 2 * n[i] - 1;
        }
    }

    for i in 0..acc.len() {
        acc[i] = (acc[i] > 0) as i32;
    }
    acc
}

/// several lines, each line has several digits
fn input() -> Vec<Binary> {
    include_str!("../../input/03.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(2).unwrap() as i32).collect())
        .collect()
}
