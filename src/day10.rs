use crate::day10::State::{Corrupted, Incomplete, Valid};
use crate::day10::Token::{
    LeftAngle, LeftCurly, LeftParen, LeftSquare, RightAngle, RightCurly, RightParen, RightSquare,
};
use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::ops::Not;

#[derive(Eq, PartialEq, Clone, Copy)]
enum Token {
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    LeftCurly,
    RightCurly,
    LeftAngle,
    RightAngle,
}

impl Token {
    fn score_corrupted(self) -> usize {
        match self {
            RightParen => 3,
            RightSquare => 57,
            RightCurly => 1197,
            RightAngle => 25137,
            _ => unreachable!(),
        }
    }

    fn score_incomplete(self) -> usize {
        match self {
            RightParen => 1,
            RightSquare => 2,
            RightCurly => 3,
            RightAngle => 4,
            _ => unreachable!(),
        }
    }
}

impl Not for Token {
    type Output = Token;

    fn not(self) -> Self::Output {
        match self {
            LeftParen => RightParen,
            LeftSquare => RightSquare,
            LeftCurly => RightCurly,
            LeftAngle => RightAngle,
            RightParen => LeftParen,
            RightSquare => LeftSquare,
            RightCurly => LeftCurly,
            RightAngle => LeftAngle,
        }
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '(' => LeftParen,
            ')' => RightParen,
            '[' => LeftSquare,
            ']' => RightSquare,
            '{' => LeftCurly,
            '}' => RightCurly,
            '<' => LeftAngle,
            '>' => RightAngle,
            _ => unreachable!(),
        }
    }
}

struct SourceLine {
    chunk: Vec<Token>,
}

impl From<&str> for SourceLine {
    fn from(l: &str) -> Self {
        SourceLine {
            chunk: l.chars().map(Token::from).collect_vec(),
        }
    }
}

enum State {
    Valid,
    Incomplete(Vec<Token>),
    Corrupted(Token),
}

struct Parser {}

impl Parser {
    fn parse(&self, lines: Vec<SourceLine>) -> Vec<State> {
        let mut states = Vec::with_capacity(lines.len());

        for line in lines {
            states.push(self.parse_line(line))
        }

        states
    }

    fn parse_line(&self, line: SourceLine) -> State {
        let mut stack = Vec::with_capacity(line.chunk.len());

        for token in line.chunk {
            match token {
                LeftAngle | LeftCurly | LeftParen | LeftSquare => stack.push(token),
                closing => match stack.pop() {
                    Some(opening) => {
                        if closing == !opening {
                            continue;
                        } else {
                            return Corrupted(closing);
                        }
                    }
                    None => unreachable!(),
                },
            }
        }

        if stack.is_empty() {
            Valid
        } else {
            Incomplete(stack)
        }
    }
}

fn parse_input_day10(input: &str) -> Vec<SourceLine> {
    input.lines().map(SourceLine::from).collect_vec()
}

#[aoc(day10, part1)]
fn day10_part1(input: &str) -> usize {
    let lines = parse_input_day10(input);

    let parser = Parser {};

    parser
        .parse(lines)
        .into_iter()
        .filter_map(|s| if let Corrupted(c) = s { Some(c) } else { None })
        .map(|c| c.score_corrupted())
        .sum()
}

#[aoc(day10, part2)]
fn day10_part2(input: &str) -> usize {
    let lines = parse_input_day10(input);
    let parser = Parser {};

    let sorted_scores = parser
        .parse(lines)
        .into_iter()
        .filter_map(|s| {
            if let Incomplete(open_gates) = s {
                Some(open_gates)
            } else {
                None
            }
        })
        .map(|open_gates| {
            open_gates
                .iter()
                .rev()
                .fold(0, |score, c| score * 5 + (!*c).score_incomplete())
        })
        .sorted()
        .collect_vec();

    sorted_scores[sorted_scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(day10_part1(input), 26397);
    }

    #[test]
    fn test_part2() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(day10_part2(input), 288957);
    }
}
