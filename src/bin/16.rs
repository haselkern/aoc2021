use core::panic;
use std::iter;

fn main() {
    let mut bits = Bits::from(include_str!("../../input/16.txt"));
    let (packet, _) = Packet::parse(&mut bits);
    println!("First solution: {}", packet.version_sum());
    println!("Second solution: {}", packet.evaluate());
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u64,
    payload: Payload,
}

#[derive(Debug, PartialEq, Eq)]
enum Payload {
    Literal(u64),
    Operator(Operation, Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, PartialEq, Eq)]
enum Length {
    Count(u64),
    Bits(u64),
}

#[derive(Debug, PartialEq, Eq)]
struct Bit(bool);

struct Bits(Box<dyn Iterator<Item = Bit>>);

impl Bit {
    const TRUE: Self = Bit(true);
    const FALSE: Self = Bit(false);
}

impl Bits {
    fn from(input: &'static str) -> Self {
        Self(Box::new(
            input
                .chars()
                .map(|c| c.to_digit(16).unwrap() as u64)
                .flat_map(|d: u64| [3, 2, 1, 0].into_iter().map(move |shift| (d >> shift) & 1))
                .map(|n| n == 1)
                .map(Bit),
        ))
    }

    /// Get the next n bits or die trying
    fn take(&mut self, n: usize) -> Bits {
        let mut b = Vec::new();
        for _ in 0..n {
            b.push(self.0.next().unwrap());
        }
        Bits(Box::new(b.into_iter()))
    }

    /// Get the next bit or panic
    fn next(&mut self) -> Bit {
        self.0.next().unwrap()
    }

    /// merge self with other.
    fn merge(self, other: Self) -> Self {
        Bits(Box::new(self.0.chain(other.0)))
    }

    /// Interpret all the bits as a number.
    fn num(self) -> u64 {
        let mut buf = 0;
        for next in self.0 {
            buf = (buf << 1) | (next.0 as u64)
        }
        buf
    }
}

impl Default for Bits {
    fn default() -> Self {
        Self(Box::new(iter::empty()))
    }
}

impl Packet {
    /// All parse methods everywhere return the parsed thing and the
    /// number of bits consumed.
    fn parse(bits: &mut Bits) -> (Self, u64) {
        let version = bits.take(3).num();
        let (payload, n) = Payload::parse(bits);
        (Self { version, payload }, n + 3)
    }

    fn version_sum(&self) -> u64 {
        let mut sum = self.version;
        if let Payload::Operator(_, packets) = &self.payload {
            for p in packets {
                sum += p.version_sum();
            }
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        match &self.payload {
            Payload::Literal(n) => *n,
            Payload::Operator(Operation::Sum, ps) => ps.iter().map(|p| p.evaluate()).sum(),
            Payload::Operator(Operation::Product, ps) => ps.iter().map(|p| p.evaluate()).product(),
            Payload::Operator(Operation::Minimum, ps) => {
                ps.iter().map(|p| p.evaluate()).min().unwrap()
            }
            Payload::Operator(Operation::Maximum, ps) => {
                ps.iter().map(|p| p.evaluate()).max().unwrap()
            }
            Payload::Operator(Operation::GreaterThan, ps) => {
                if ps[0].evaluate() > ps[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Payload::Operator(Operation::LessThan, ps) => {
                if ps[0].evaluate() < ps[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Payload::Operator(Operation::EqualTo, ps) => {
                if ps[0].evaluate() == ps[1].evaluate() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl Payload {
    fn parse(bits: &mut Bits) -> (Self, u64) {
        let op = match bits.take(3).num() {
            0 => Some(Operation::Sum),
            1 => Some(Operation::Product),
            2 => Some(Operation::Minimum),
            3 => Some(Operation::Maximum),
            4 => None,
            5 => Some(Operation::GreaterThan),
            6 => Some(Operation::LessThan),
            7 => Some(Operation::EqualTo),
            n => panic!("unknown operation: {}", n),
        };
        let (p, n) = match op {
            None => Self::parse_literal(bits),
            Some(op) => Self::parse_operator(op, bits),
        };
        (p, n + 3)
    }

    fn parse_operator(op: Operation, bits: &mut Bits) -> (Self, u64) {
        let mut counter = 0;
        let (required, b) = Length::parse(bits);
        counter += b;

        let mut result = Vec::new();
        let mut child_bit_counter = 0;

        loop {
            // Check if we parsed enough things
            if match required {
                Length::Count(c) => result.len() as u64 >= c,
                Length::Bits(b) => child_bit_counter >= b,
            } {
                break;
            }

            // Parse child
            let (child, b) = Packet::parse(bits);
            child_bit_counter += b;
            result.push(child);
        }

        counter += child_bit_counter;
        (Self::Operator(op, result), counter)
    }

    fn parse_literal(bits: &mut Bits) -> (Self, u64) {
        let mut more = Bit::TRUE;
        let mut result = Bits::default();
        let mut counter  = 0;
        while more == Bit::TRUE {
            let mut chunk = bits.take(5);
            counter += 5;
            more = chunk.next();
            result = result.merge(chunk);
        }
        (Self::Literal(result.num()), counter)
    }
}

impl Length {
    fn parse(bits: &mut Bits) -> (Self, u64) {
        match bits.next() {
            Bit::FALSE => (Length::Bits(bits.take(15).num()), 16),
            Bit::TRUE => (Length::Count(bits.take(11).num()), 12),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Bits, Operation, Packet, Payload};

    #[test]
    fn nums() {
        assert_eq!(Bits::from("F").take(4).num(), 15);
        assert_eq!(Bits::from("F").take(2).num(), 3);
        assert_eq!(Bits::from("A").take(4).num(), 10);
        assert_eq!(Bits::from("A").take(2).num(), 2);
        assert_eq!(Bits::from("ABC").take(12).num(), 0xABC);
    }

    #[test]
    fn example_literal() {
        let mut bits = Bits::from("D2FE28");
        let (actual, _) = Packet::parse(&mut bits);
        let expected = Packet {
            version: 6,
            payload: Payload::Literal(2021),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn example_operator() {
        let mut bits = Bits::from("38006F45291200");
        let (actual, _) = Packet::parse(&mut bits);
        let expected = Packet {
            version: 1,
            payload: Payload::Operator(
                Operation::LessThan,
                vec![
                    Packet {
                        version: 6,
                        payload: Payload::Literal(10),
                    },
                    Packet {
                        version: 2,
                        payload: Payload::Literal(20),
                    },
                ],
            ),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn example_sums() {
        let mut bits = Bits::from("8A004A801A8002F478");
        let (packet, _) = Packet::parse(&mut bits);
        assert_eq!(packet.version_sum(), 16);

        let mut bits = Bits::from("620080001611562C8802118E34");
        let (packet, _) = Packet::parse(&mut bits);
        assert_eq!(packet.version_sum(), 12);

        let mut bits = Bits::from("C0015000016115A2E0802F182340");
        let (packet, _) = Packet::parse(&mut bits);
        assert_eq!(packet.version_sum(), 23);

        let mut bits = Bits::from("A0016C880162017C3686B18A3D4780");
        let (packet, _) = Packet::parse(&mut bits);
        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn example_evaluations() {
        let mut bits = Bits::from("C200B40A82");
        let (packet, _) = Packet::parse(&mut bits);
        assert_eq!(packet.evaluate(), 3);

        let mut bits = Bits::from("04005AC33890");
        let (packet, _) = Packet::parse(&mut bits);
        assert_eq!(packet.evaluate(), 54);

        let mut bits = Bits::from("880086C3E88112");
        let (packet, _) = Packet::parse(&mut bits);
        assert_eq!(packet.evaluate(), 7);

        let mut bits = Bits::from("D8005AC2A8F0");
        let (packet, _) = Packet::parse(&mut bits);
        assert_eq!(packet.evaluate(), 1);

        let mut bits = Bits::from("F600BC2D8F");
        let (packet, _) = Packet::parse(&mut bits);
        assert_eq!(packet.evaluate(), 0);

        let mut bits = Bits::from("9C005AC2F8F0");
        let (packet, _) = Packet::parse(&mut bits);
        assert_eq!(packet.evaluate(), 0);

        let mut bits = Bits::from("9C0141080250320F1802104A08");
        let (packet, _) = Packet::parse(&mut bits);
        assert_eq!(packet.evaluate(), 1);
    }
}
