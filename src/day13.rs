use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn transpose_x(&self, x: usize) -> Point {
        Point {
            x: x * 2 - self.x,
            y: self.y,
        }
    }

    fn transpose_y(&self, y: usize) -> Point {
        Point {
            x: self.x,
            y: y * 2 - self.y,
        }
    }
}

#[derive(Debug)]
enum Axis {
    XAxis,
    YAxis,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    value: usize,
}

impl From<(&str, &str)> for Point {
    fn from(t: (&str, &str)) -> Self {
        Point {
            x: t.0.parse().unwrap(),
            y: t.1.parse().unwrap(),
        }
    }
}

impl From<(&str, &str)> for Fold {
    fn from(t: (&str, &str)) -> Self {
        Fold {
            axis: match t.0 {
                "x" => Axis::XAxis,
                "y" => Axis::YAxis,
                _ => unreachable!(),
            },
            value: t.1.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct ParsedInput {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

#[aoc_generator(day13)]
fn parse_input_day13(input: &str) -> ParsedInput {
    let fold_regex = Regex::new(r"(?P<axis>[xy])=(?P<value>\d+)").unwrap();
    let mut parts = input.split("\n\n");

    ParsedInput {
        points: parts
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                l.split(',')
                    .collect_tuple::<(&str, &str)>()
                    .map(Point::from)
                    .unwrap()
            })
            .collect(),
        folds: fold_regex
            .captures_iter(parts.next().unwrap())
            .map(|capture| {
                Fold::from((
                    capture.name("axis").unwrap().as_str(),
                    capture.name("value").unwrap().as_str(),
                ))
            })
            .collect(),
    }
}

#[aoc(day13, part1)]
fn day13_part1(input: &ParsedInput) -> Option<usize> {
    Some(fold(&input.points, &input.folds[..1]).len())
}

#[aoc(day13, part2)]
fn day13_part2(input: &ParsedInput) -> Option<usize> {
    let points = fold(&input.points, &input.folds);

    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut grid = vec![vec![' '; max_x + 1]; max_y + 1];

    points.iter().for_each(|p| grid[p.y][p.x] = '#');

    println!("{}", grid.iter().map(|r| r.iter().join("")).join("\n"));

    Some(1)
}

fn fold(points: &[Point], folds: &[Fold]) -> Vec<Point> {
    let mut points = points.to_vec();

    let mut grid = points.iter().cloned().collect::<HashSet<_>>();

    for fold in folds.iter() {
        for point in points {
            match *fold {
                Fold {
                    axis: Axis::XAxis,
                    value: n,
                } => {
                    if point.x > n {
                        grid.remove(&point);
                        grid.insert(point.transpose_x(n));
                    }
                }
                Fold {
                    axis: Axis::YAxis,
                    value: n,
                } => {
                    if point.y > n {
                        grid.remove(&point);
                        grid.insert(point.transpose_y(n));
                    }
                }
            }
        }

        points = grid.iter().cloned().collect_vec();
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        assert_eq!(day13_part1(&parse_input_day13(input)), Some(17));
    }

    #[test]
    fn test_part2() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        /*
        ###..####.#..#.###...##....##.####.#..#
        #..#.#....#.#..#..#.#..#....#....#.#..#
        ###..###..##...#..#.#.......#...#..#..#
        #..#.#....#.#..###..#.......#..#...#..#
        #..#.#....#.#..#.#..#..#.#..#.#....#..#
        ###..#....#..#.#..#..##...##..####..##.
        */

        assert_eq!(day13_part2(&parse_input_day13(input)), Some(1));
    }
}
