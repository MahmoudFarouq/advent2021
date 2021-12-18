use crate::day18::Number::{Pair, Regular};
use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::Formatter;
use std::ops::Add;
use std::str::Chars;

#[derive(Eq, PartialEq, Clone)]
enum Number {
    Regular(usize),
    Pair(Box<Number>, Box<Number>),
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut t = Pair(self.into(), rhs.into());
        t.reduce();
        t
    }
}

impl Number {
    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn leftmost(&mut self) -> &mut usize {
        match self {
            Regular(n) => n,
            Pair(l, _) => l.leftmost(),
        }
    }

    fn rightmost(&mut self) -> &mut usize {
        match self {
            Regular(n) => n,
            Pair(_, r) => r.rightmost(),
        }
    }

    fn explode(&mut self) -> bool {
        match self {
            Pair(l, r) => {
                l._explode(1, None, Some(r.leftmost())) || r._explode(1, Some(l.rightmost()), None)
            }
            Regular(_) => false,
        }
    }

    fn _explode(
        &mut self,
        level: usize,
        left_value: Option<&mut usize>,
        right_value: Option<&mut usize>,
    ) -> bool {
        if level == 4 {
            match self {
                Regular(_) => false,
                Pair(l, r) => {
                    if let Regular(l) = l.as_ref() {
                        if let Some(left_value) = left_value {
                            *left_value += l;
                        }
                    }

                    if let Regular(r) = r.as_ref() {
                        if let Some(right_value) = right_value {
                            *right_value += r;
                        }
                    }

                    *self = Regular(0);

                    true
                }
            }
        } else {
            match self {
                Regular(_) => false,
                Pair(l, r) => {
                    l._explode(level + 1, left_value, Some(r.leftmost()))
                        || r._explode(level + 1, Some(l.rightmost()), right_value)
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Pair(l, r) => l.split() || r.split(),
            Regular(ref n) if *n >= 10 => {
                *self = Pair(Regular(n / 2).into(), Regular(n / 2 + n % 2).into());
                true
            }
            _ => false,
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
            Regular(n) => *n,
        }
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Regular(n) => write!(f, "{:?}", n),
            Pair(l, r) => write!(f, "[{:?}, {:?}]", *l, *r),
        }
    }
}

impl From<&str> for Number {
    fn from(l: &str) -> Self {
        let mut chars = l.chars();
        Self::from_chars(&mut chars).unwrap()
    }
}

impl Number {
    fn from_chars(l: &mut Chars) -> Option<Self> {
        l.next().and_then(|c| match c {
            '[' => {
                let left = Self::from_chars(l);
                let _comma = Self::from_chars(l);
                let right = Self::from_chars(l);
                let _closing = Self::from_chars(l);
                Some(Pair(left.unwrap().into(), right.unwrap().into()))
            }
            d if c.is_digit(10) => Some(Regular(d.to_digit(10).unwrap() as usize)),
            ']' | ',' => None,
            _ => unreachable!(),
        })
    }
}

#[aoc_generator(day18)]
fn parse_input_day18(input: &str) -> Vec<Number> {
    input.lines().map(Number::from).collect()
}

#[aoc(day18, part1)]
fn day18_part1(input: &[Number]) -> Option<usize> {
    let mut result = input[0].clone();

    for number in input.iter().cloned().skip(1) {
        result = result + number;
    }

    result.magnitude().into()
}

#[aoc(day18, part2)]
fn day18_part2(input: &[Number]) -> Option<usize> {
    let mut sum = 0;

    for first in input.iter() {
        for second in input.iter() {
            sum = sum.max((first.clone() + second.clone()).magnitude());
        }
    }

    sum.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(day18_part1(&parse_input_day18(input)), Some(4140));
    }

    #[test]
    fn test_part2() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(day18_part2(&parse_input_day18(input)), Some(3993));
    }
}
