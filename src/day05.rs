use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&mut self, diff_x: i32, diff_y: i32) {
        self.x += diff_x;
        self.y += diff_y;
    }
}

#[derive(Eq, PartialEq)]
enum Strategy {
    Vh,
    Vhd,
}

#[derive(Debug, Eq, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn points_in_between(&self, s: Strategy) -> Vec<Point> {
        let diff_x = match self.start.x.cmp(&self.end.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let diff_y = match self.start.y.cmp(&self.end.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        if s == Strategy::Vh && diff_x != 0 && diff_y != 0 {
            return Vec::new();
        }

        let mut moving_point = self.start;

        let mut result = vec![moving_point];

        while moving_point != self.end {
            moving_point.add(diff_x, diff_y);
            result.push(moving_point);
        }

        result
    }
}

#[aoc_generator(day5)]
fn parse_input_day5(input: &str) -> Vec<Line> {
    let regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    input
        .lines()
        .map(|l| {
            let iter = regex.captures(l).unwrap();
            Line {
                start: Point {
                    x: iter.get(1).unwrap().as_str().parse().unwrap(),
                    y: iter.get(2).unwrap().as_str().parse().unwrap(),
                },
                end: Point {
                    x: iter.get(3).unwrap().as_str().parse().unwrap(),
                    y: iter.get(4).unwrap().as_str().parse().unwrap(),
                },
            }
        })
        .collect_vec()
}

#[aoc(day5, part1)]
fn day5_part1(input: &[Line]) -> Option<usize> {
    input
        .iter()
        .fold(HashMap::<Point, u32>::new(), |mut diagram, line| {
            line.points_in_between(Strategy::Vh)
                .iter()
                .for_each(|p| *diagram.entry(*p).or_default() += 1);
            diagram
        })
        .iter()
        .filter(|(_, v)| v > &&1)
        .count()
        .into()
}

#[aoc(day5, part2)]
fn day5_part2(input: &[Line]) -> Option<usize> {
    input
        .iter()
        .fold(HashMap::<Point, u32>::new(), |mut diagram, line| {
            line.points_in_between(Strategy::Vhd)
                .iter()
                .for_each(|p| *diagram.entry(*p).or_default() += 1);
            diagram
        })
        .iter()
        .filter(|(_, v)| v > &&1)
        .count()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(
            parse_input_day5(&input),
            vec![
                Line {
                    start: Point { x: 0, y: 9 },
                    end: Point { x: 5, y: 9 }
                },
                Line {
                    start: Point { x: 8, y: 0 },
                    end: Point { x: 0, y: 8 }
                },
                Line {
                    start: Point { x: 9, y: 4 },
                    end: Point { x: 3, y: 4 }
                },
                Line {
                    start: Point { x: 2, y: 2 },
                    end: Point { x: 2, y: 1 }
                },
                Line {
                    start: Point { x: 7, y: 0 },
                    end: Point { x: 7, y: 4 }
                },
                Line {
                    start: Point { x: 6, y: 4 },
                    end: Point { x: 2, y: 0 }
                },
                Line {
                    start: Point { x: 0, y: 9 },
                    end: Point { x: 2, y: 9 }
                },
                Line {
                    start: Point { x: 3, y: 4 },
                    end: Point { x: 1, y: 4 }
                },
                Line {
                    start: Point { x: 0, y: 0 },
                    end: Point { x: 8, y: 8 }
                },
                Line {
                    start: Point { x: 5, y: 5 },
                    end: Point { x: 8, y: 2 }
                },
            ]
        );
    }

    #[test]
    fn test_line_points_in_between() {
        let l = Line {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 9 },
        };

        assert_eq!(
            l.points_in_between(Strategy::Vh),
            vec![
                Point { x: 0, y: 9 },
                Point { x: 1, y: 9 },
                Point { x: 2, y: 9 },
                Point { x: 3, y: 9 },
                Point { x: 4, y: 9 },
                Point { x: 5, y: 9 },
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(day5_part1(&parse_input_day5(&input)), Some(5));
    }

    #[test]
    fn test_part2() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(day5_part2(&parse_input_day5(&input)), Some(12));
    }
}
