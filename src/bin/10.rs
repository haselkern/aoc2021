use std::collections::VecDeque;

type Line = VecDeque<char>;

fn main() {
    let (score, incomplete_lines) = part1();
    println!("First solution: {}", score);
    println!("Second solution: {}", part2(incomplete_lines));
}

fn part1() -> (usize, Vec<Line>) {
    let mut score = 0;

    let mut input = input();
    input.retain(|f| {
        let mut stack = Vec::new();
        let mut line = f.clone();
        while let Some(next) = line.pop_front() {
            if is_open(next) {
                stack.push(next);
            } else {
                let popped = stack.pop().unwrap();
                if !is_pair(popped, next) {
                    // This line is corrupted.
                    score += char_score_invalid(next);
                    return false;
                }
            }
        }
        true
    });

    (score, input)
}

fn part2(lines: Vec<Line>) -> usize {
    let mut scores = Vec::new();

    for mut line in lines {
        let mut stack = Vec::new();
        while let Some(next) = line.pop_front() {
            if is_open(next) {
                stack.push(next);
            } else {
                stack.pop();
            }
        }

        let mut score = 0;
        while let Some(next) = stack.pop() {
            score *= 5;
            score += char_score_incomplete(next);
        }
        scores.push(score);
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn is_open(c: char) -> bool {
    matches!(c, '<' | '{' | '[' | '(')
}

fn is_pair(open: char, close: char) -> bool {
    matches!(
        (open, close),
        ('<', '>') | ('{', '}') | ('(', ')') | ('[', ']')
    )
}

fn char_score_invalid(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        c => panic!("invalid char for score '{}'", c),
    }
}

fn char_score_incomplete(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        c => panic!("invalid char for score '{}'", c),
    }
}

fn input() -> Vec<Line> {
    include_str!("../../input/10.txt")
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}
