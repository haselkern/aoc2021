fn main() {
    let mut pos = State::default();
    pos.apply(input());
    println!("First solution: {:?} => {}", pos, pos.position * pos.depth);

    let mut pos = State::default();
    pos.apply_new(input());
    println!("Second solution: {:?} => {}", pos, pos.position * pos.depth);
}

#[derive(Debug, Default)]
struct State {
    position: i32,
    depth: i32,
    aim: i32,
}

impl State {
    fn apply(&mut self, instructions: impl Iterator<Item = Instruction>) {
        for i in instructions {
            match i.direction {
                "up" => self.depth -= i.distance,
                "down" => self.depth += i.distance,
                "forward" => self.position += i.distance,
                d => panic!("unknown direction: {}", d),
            }
        }
    }

    fn apply_new(&mut self, instructions: impl Iterator<Item = Instruction>) {
        for i in instructions {
            match i.direction {
                "up" => self.aim -= i.distance,
                "down" => self.aim += i.distance,
                "forward" => {
                    self.position += i.distance;
                    self.depth += self.aim * i.distance;
                }
                d => panic!("unknown direction: {}", d),
            }
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: &'static str,
    distance: i32,
}

fn input() -> impl Iterator<Item = Instruction> {
    include_str!("../../input/02.txt")
        .lines()
        .filter_map(|l| l.split_once(" "))
        .map(|(d, n)| Instruction {
            direction: d,
            distance: n.parse().unwrap(),
        })
}
