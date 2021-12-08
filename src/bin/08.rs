fn main() {
    println!("First solution: {}", part1());
    println!("Second solution: {}", part2());
}

fn part1() -> usize {
    input()
        .iter()
        .flat_map(|p| &p.challenge)
        .filter(|s| matches!(s.count_ones(), 2 | 4 | 3 | 7))
        .count()
}

fn part2() -> usize {
    input().iter().map(Puzzle::solve).sum()
}

type Digit = u8;

/// Parse chars a to g to bit flags.
fn parse_char(s: char) -> Digit {
    match s {
        'a' => 1,
        'b' => 2,
        'c' => 4,
        'd' => 8,
        'e' => 16,
        'f' => 32,
        'g' => 64,
        err => panic!("invalid digit '{}'", err),
    }
}

fn parse_digit(s: &str) -> Digit {
    s.chars().fold(0, |a, b| a | parse_char(b))
}

#[derive(Debug)]
struct Puzzle {
    hints: Vec<Digit>,
    challenge: Vec<Digit>,
}

impl Puzzle {
    fn solve(&self) -> usize {
        // Easy numbers have unique numbers of segments
        let one = assert_single(self.by_segments(2));
        let four = assert_single(self.by_segments(4));
        let seven = assert_single(self.by_segments(3));
        let eight = assert_single(self.by_segments(7));

        // Only three overlaps one
        let three = self.by_segments(5).find(|&n| n & one == one).unwrap();

        // Only nine overlaps four and seven
        let nine = assert_single(
            self.by_segments(6)
                .filter(|&n| n & four == four && n & seven == seven),
        );

        // Find zero. It has 6 segments, is not nine and overlaps one.
        let zero = assert_single(self.by_segments(6).filter(|&n| n != nine && n & one == one));

        // six is the only 6 segment number that's not nine or zero
        let six = assert_single(self.by_segments(6).filter(|&n| n != nine && n != zero));

        // To get five, join one and six. That is a single segment that only five and three have, but we know three.
        let five = assert_single(
            self.by_segments(5)
                .filter(|&n| n & one & six > 0 && n != three),
        );

        // Only two remains
        let two = assert_single(self.by_segments(5).filter(|&n| n != five && n != three));

        let nums = [zero, one, two, three, four, five, six, seven, eight, nine];

        // We can now decode the challenge
        let mut result = 0;
        for &d in &self.challenge {
            result *= 10;
            for (i, &n) in nums.iter().enumerate() {
                if n == d {
                    result += i;
                }
            }
        }
        result
    }

    fn by_segments(&self, n: u32) -> impl Iterator<Item = Digit> {
        self.hints
            .clone()
            .into_iter()
            .filter(move |d| d.count_ones() == n)
    }

    fn from_str(s: &str) -> Self {
        let (digits, test) = s.split_once('|').unwrap();
        Self {
            hints: digits.split_ascii_whitespace().map(parse_digit).collect(),
            challenge: test.split_ascii_whitespace().map(parse_digit).collect(),
        }
    }
}

fn assert_single(mut it: impl Iterator<Item = Digit>) -> Digit {
    let result = it.next().unwrap();
    assert_eq!(it.next(), None);
    result
}

fn input() -> Vec<Puzzle> {
    include_str!("../../input/08.txt")
        .lines()
        .map(Puzzle::from_str)
        .collect()
}
