use crate::day16::Bit::{One, Zero};
use crate::day16::PacketType::Operator;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::str::Chars;

#[derive(Eq, PartialEq, Debug, Clone)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    fn into_string(self) -> String {
        match self {
            Zero => "0",
            One => "1",
        }
        .to_string()
    }
}

struct BitsIterator {
    num: usize,
    mask: usize,
}

impl BitsIterator {
    fn new_with_length(num: usize, length: u32) -> Self {
        BitsIterator {
            num,
            mask: 1 << (length - 1),
        }
    }
}

impl Iterator for BitsIterator {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mask {
            0 => None,
            _ => {
                let r = self.num & self.mask;
                self.mask >>= 1;
                Some(if r == 0 { Zero } else { One })
            }
        }
    }
}

struct Scanner<'a> {
    chars: Chars<'a>,
    nibble_iterator: BitsIterator,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_nibble = chars.next().unwrap().to_digit(16).unwrap() as usize;
        Scanner {
            chars,
            nibble_iterator: BitsIterator::new_with_length(current_nibble, 4),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        self.nibble_iterator.next().or_else(|| {
            self.chars.next().map(|c| {
                self.nibble_iterator =
                    BitsIterator::new_with_length(c.to_digit(16).unwrap() as usize, 4);
                self.nibble_iterator.next().unwrap()
            })
        })
    }
}

type PacketVersion = u8;

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<u8> for Op {
    fn from(v: u8) -> Self {
        match v {
            0 => Op::Sum,
            1 => Op::Product,
            2 => Op::Minimum,
            3 => Op::Maximum,
            5 => Op::GreaterThan,
            6 => Op::LessThan,
            7 => Op::EqualTo,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Operator { op: Op, packets: Vec<Packet> },
}

#[derive(Debug)]
struct Packet {
    version: PacketVersion,
    packet_type: PacketType,
    size: usize,
}

struct Parser<'a> {
    scanner: Scanner<'a>,
    consumed_bits: usize,
}

impl<'a> Parser<'a> {
    fn new(scanner: Scanner<'a>) -> Self {
        Parser {
            scanner,
            consumed_bits: 0,
        }
    }

    fn parse(&mut self) -> Packet {
        self.parse_packet()
    }

    fn parse_packet(&mut self) -> Packet {
        let started_at = self.consumed_count();

        let version = self.parse_version();
        let packet_type = self.parse_type();

        let ended_at = self.consumed_count();

        Packet {
            version,
            packet_type,
            size: ended_at - started_at,
        }
    }

    fn parse_version(&mut self) -> PacketVersion {
        const BITS_COUNT: usize = 3;
        let bits = self.eat(BITS_COUNT);

        u8::from_str_radix(&self.concat(bits), 2).unwrap()
    }

    fn parse_type(&mut self) -> PacketType {
        const BITS_COUNT: usize = 3;
        let bits = self.eat(BITS_COUNT);

        let v = u8::from_str_radix(&self.concat(bits), 2).unwrap();

        if v == 4 {
            self.parse_literal_packet()
        } else {
            self.parse_operator_packet(v)
        }
    }

    fn parse_literal_packet(&mut self) -> PacketType {
        let mut builder = Vec::new();

        loop {
            let bits = self.eat(5);

            builder.extend(bits.iter().cloned().skip(1));

            if bits[0] == Zero {
                break;
            }
        }

        let v = usize::from_str_radix(&self.concat(builder), 2).unwrap();

        PacketType::Literal(v)
    }

    fn parse_operator_packet(&mut self, value: u8) -> PacketType {
        let length_type_id = self.eat(1).first().cloned().unwrap();

        let packets = match length_type_id {
            Zero => {
                let total_length_in_bits = self.eat(15);
                let total_length_in_bits =
                    usize::from_str_radix(&self.concat(total_length_in_bits), 2).unwrap();

                let mut packets = Vec::new();
                let mut read_until_now = 0;
                while read_until_now < total_length_in_bits {
                    let p = self.parse_packet();
                    read_until_now += p.size;

                    packets.push(p);
                }

                packets
            }
            One => {
                let number_of_inner_sub_packets = self.eat(11);
                let number_of_inner_sub_packets =
                    usize::from_str_radix(&self.concat(number_of_inner_sub_packets), 2).unwrap();

                let mut packets = Vec::with_capacity(number_of_inner_sub_packets);
                for _ in 0..number_of_inner_sub_packets {
                    packets.push(self.parse_packet())
                }

                packets
            }
        };

        PacketType::Operator {
            op: value.into(),
            packets,
        }
    }

    fn eat(&mut self, n: usize) -> Vec<Bit> {
        let mut builder = Vec::with_capacity(n);

        for _ in 0..n {
            match self.scanner.next() {
                Some(b) => {
                    builder.push(b);
                }
                None => {
                    panic!("Oops!")
                }
            }
        }

        self.consumed_bits += n;

        builder
    }

    fn concat(&self, bits: Vec<Bit>) -> String {
        bits.iter().cloned().map(Bit::into_string).join("")
    }

    fn consumed_count(&self) -> usize {
        self.consumed_bits
    }
}

struct Interpreter {}

impl Interpreter {
    fn evaluate(&self, p: &Packet) -> usize {
        match p.packet_type {
            PacketType::Literal(v) => v,
            PacketType::Operator { .. } => self.evaluate_type_operator(&p.packet_type),
        }
    }

    fn evaluate_type_operator(&self, packet_type: &PacketType) -> usize {
        if let PacketType::Operator { op, packets } = packet_type {
            match op {
                Op::Sum => self.evaluate_op_sum(packets),
                Op::Product => self.evaluate_op_product(packets),
                Op::Minimum => self.evaluate_op_minimum(packets),
                Op::Maximum => self.evaluate_op_maximum(packets),
                Op::GreaterThan => self.evaluate_op_greater_than(packets),
                Op::LessThan => self.evaluate_op_less_than(packets),
                Op::EqualTo => self.evaluate_op_equal_to(packets),
            }
        } else {
            panic!("invalid packet type");
        }
    }
    fn evaluate_op_sum(&self, packets: &[Packet]) -> usize {
        packets.iter().map(|p| self.evaluate(p)).sum()
    }
    fn evaluate_op_product(&self, packets: &[Packet]) -> usize {
        packets.iter().map(|p| self.evaluate(p)).product()
    }
    fn evaluate_op_minimum(&self, packets: &[Packet]) -> usize {
        packets.iter().map(|p| self.evaluate(p)).min().unwrap()
    }
    fn evaluate_op_maximum(&self, packets: &[Packet]) -> usize {
        packets.iter().map(|p| self.evaluate(p)).max().unwrap()
    }
    fn evaluate_op_greater_than(&self, packets: &[Packet]) -> usize {
        if packets.len() != 2 {
            panic!("invalid packets length for greater op")
        }

        (self.evaluate(&packets[0]) > self.evaluate(&packets[1])) as usize
    }
    fn evaluate_op_less_than(&self, packets: &[Packet]) -> usize {
        if packets.len() != 2 {
            panic!("invalid packets length for less op")
        }

        (self.evaluate(&packets[0]) < self.evaluate(&packets[1])) as usize
    }
    fn evaluate_op_equal_to(&self, packets: &[Packet]) -> usize {
        if packets.len() != 2 {
            panic!("invalid packets length for equal op")
        }

        (self.evaluate(&packets[0]) == self.evaluate(&packets[1])) as usize
    }
}

#[aoc(day16, part1)]
fn day16_part1(input: &str) -> Option<usize> {
    let scanner = Scanner::new(input);
    let mut parser = Parser::new(scanner);
    let packet = parser.parse();

    let sum_version = sum_versions(&packet);

    Some(sum_version)
}

#[aoc(day16, part2)]
fn day16_part2(input: &str) -> Option<usize> {
    let scanner = Scanner::new(input);
    let mut parser = Parser::new(scanner);
    let packet = parser.parse();

    let interpreter = Interpreter {};

    Some(interpreter.evaluate(&packet))
}

fn sum_versions(p: &Packet) -> usize {
    p.version as usize
        + match p.packet_type {
            Operator { ref packets, op: _ } => {
                packets.iter().fold(0, |sum, p| sum + sum_versions(p))
            }
            _ => 0,
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_iterator() {
        let scanner = Scanner::new("DA");
        assert_eq!(
            scanner.collect::<Vec<Bit>>(),
            vec![One, One, Zero, One, One, Zero, One, Zero]
        );

        let scanner = Scanner::new("D2FE28");
        assert_eq!(
            scanner.collect::<Vec<Bit>>(),
            vec![
                One, One, Zero, One, Zero, Zero, One, Zero, One, One, One, One, One, One, One,
                Zero, Zero, Zero, One, Zero, One, Zero, Zero, Zero,
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = "8A004A801A8002F478";
        assert_eq!(day16_part1(input), Some(16));

        let input = "620080001611562C8802118E34";
        assert_eq!(day16_part1(input), Some(12));

        let input = "C0015000016115A2E0802F182340";
        assert_eq!(day16_part1(input), Some(23));

        let input = "A0016C880162017C3686B18A3D4780";
        assert_eq!(day16_part1(input), Some(31));
    }

    #[test]
    fn test_part2() {
        let input = "9C0141080250320F1802104A08";
        assert_eq!(day16_part2(input), Some(1));
    }
}
