use std::ops::Add;

fn main() {
    println!(
        "First solution: {}",
        part1(include_str!("../../input/18.txt"))
    );
    println!(
        "Second solution: {}",
        part2(include_str!("../../input/18.txt"))
    );
}

fn part1(s: &str) -> u32 {
    s.lines()
        .map(Number::from)
        .reduce(|a, b| a + b)
        .unwrap()
        .magnitude()
}

fn part2(s: &str) -> u32 {
    let numbers: Vec<Number> = s.lines().map(Number::from).collect();
    let mut max = 0;

    for a in &numbers {
        for b in &numbers {
            let sum = (a.clone() + b.clone()).magnitude();
            max = max.max(sum);
        }
    }

    max
}

#[derive(Debug, Clone, Eq)]
struct Number(Vec<Token>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open,
    Close,
    Num(u32),
}

impl Token {
    /// assume that this token is a number and return the value or panic.
    fn num(&self) -> u32 {
        match self {
            Token::Num(n) => *n,
            t => panic!("expected Token::Num, but got {:?}", t),
        }
    }

    fn num_mut(&mut self) -> &mut u32 {
        match self {
            Token::Num(n) => n,
            t => panic!("expected Token::Num, but got {:?}", t),
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut v = vec![Token::Open];
        v.extend(self.0);
        v.extend(rhs.0);
        v.push(Token::Close);
        Self(v).reduce()
    }
}

impl From<&str> for Number {
    fn from(s: &str) -> Self {
        Self(
            s.chars()
                .filter_map(|c| match c {
                    '[' => Some(Token::Open),
                    ']' => Some(Token::Close),
                    ',' => None, // commas do not matter after parsing
                    d => match d.to_digit(10) {
                        Some(d) => Some(Token::Num(d)),
                        None => panic!("unknown char '{}'", d),
                    },
                })
                .collect(),
        )
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Number {
    fn magnitude(&self) -> u32 {
        let mut stack: Vec<u32> = Vec::new();

        for tok in &self.0 {
            match tok {
                Token::Open => continue,
                Token::Close => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(3 * a + 2 * b);
                }
                Token::Num(n) => stack.push(*n),
            }
        }

        stack.pop().unwrap()
    }

    fn reduce(&self) -> Self {
        let mut result = self.clone();

        loop {
            let (num, exploded) = result.explode();
            if exploded {
                result = num;
            } else {
                let (num, split) = num.split();
                if split {
                    result = num;
                } else {
                    return result;
                }
            }
        }
    }

    /// Perform the leftmost explosion, if any. Returns true if any explosions happened.
    fn explode(&self) -> (Self, bool) {
        let mut explode_happened = false;
        let mut depth = 0;

        // Keep track of the last num to move a value backwards.
        let mut last_num_idx: Option<usize> = None;
        // Carry a value to the next num
        let mut carrying: Option<u32> = None;

        let mut result: Vec<Token> = Vec::new();
        let mut tokens = self.0.iter().copied();

        while let Some(tok) = tokens.next() {
            match tok {
                Token::Open if depth >= 4 && !explode_happened => {
                    // Explode!
                    explode_happened = true;
                    let left = tokens.next().unwrap().num();
                    let right = tokens.next().unwrap().num();
                    let _close = tokens.next();

                    if let Some(i) = last_num_idx {
                        *result[i].num_mut() += left;
                    }
                    carrying = Some(right);

                    result.push(Token::Num(0));
                }
                Token::Open => {
                    depth += 1;
                    result.push(Token::Open);
                }
                Token::Close => {
                    depth -= 1;
                    result.push(Token::Close);
                }
                Token::Num(n) => {
                    last_num_idx = Some(result.len());
                    let n = n + carrying.take().unwrap_or(0);
                    result.push(Token::Num(n));
                }
            }
        }

        (Self(result), explode_happened)
    }

    /// Perform the leftmost split, if any. Returns true if any splits happened.
    fn split(&self) -> (Self, bool) {
        let mut split_happened = false;
        let mut result = Vec::new();

        for tok in self.0.iter().copied() {
            match tok {
                Token::Num(n) if n > 9 && !split_happened => {
                    // Split!
                    split_happened = true;
                    let (a, b) = split(n);
                    result.extend([Token::Open, Token::Num(a), Token::Num(b), Token::Close]);
                }
                tok => result.push(tok),
            }
        }

        (Self(result), split_happened)
    }
}

fn split(n: u32) -> (u32, u32) {
    if n % 2 == 0 {
        (n / 2, n / 2)
    } else {
        (n / 2, n / 2 + 1)
    }
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, Number};

    #[test]
    fn magnitude() {
        assert_eq!(Number::from("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(
            Number::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
            1384
        );
        assert_eq!(
            Number::from("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(),
            445
        );
        assert_eq!(
            Number::from("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(),
            791
        );
        assert_eq!(
            Number::from("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(),
            1137
        );
        assert_eq!(
            Number::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
            3488
        );
    }

    #[test]
    fn explode() {
        let expected = Number::from("[[[[0,9],2],3],4]");
        let actual = Number::from("[[[[[9,8],1],2],3],4]").reduce();
        assert_eq!(expected, actual)
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(include_str!("../../input/18-test.txt")), 4140);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(include_str!("../../input/18-test.txt")), 3993);
    }
}
