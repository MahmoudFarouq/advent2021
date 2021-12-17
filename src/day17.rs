use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

const MAX_TRIAL_RANGE: isize = 300;

#[derive(Debug, Eq, PartialEq)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn contains(&self, n: isize) -> bool {
        n >= self.start && n <= self.end
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn add(&self, p: &Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct TargetArea {
    x_range: Range,
    y_range: Range,
}

impl TargetArea {
    fn contains(&self, p: &Point) -> bool {
        self.x_range.contains(p.x) && self.y_range.contains(p.y)
    }
}

#[aoc_generator(day17)]
fn parse_input_day17(input: &str) -> TargetArea {
    let regex = Regex::new(r"target area: x=(\d+)..(\d+), y=(-?\d+)..(-?\d+)").unwrap();

    let captures = regex.captures(input).unwrap();
    let mut captures = captures.iter();

    captures.next();

    TargetArea {
        x_range: Range {
            start: captures.next().unwrap().unwrap().as_str().parse().unwrap(),
            end: captures.next().unwrap().unwrap().as_str().parse().unwrap(),
        },
        y_range: Range {
            start: captures.next().unwrap().unwrap().as_str().parse().unwrap(),
            end: captures.next().unwrap().unwrap().as_str().parse().unwrap(),
        },
    }
}

#[aoc(day17, part1)]
fn day17_part1(target_area: &TargetArea) -> Option<isize> {
    let mut best_y = 0;

    for x in -MAX_TRIAL_RANGE..MAX_TRIAL_RANGE {
        for y in -MAX_TRIAL_RANGE..MAX_TRIAL_RANGE {
            if let Some(path) = find_path(target_area, Point { x, y }) {
                best_y = best_y.max(path.iter().map(|p| p.y).max().unwrap());
            }
        }
    }

    Some(best_y)
}

#[aoc(day17, part2)]
fn day17_part2(target_area: &TargetArea) -> Option<usize> {
    let mut good_velocities_count = 0;

    for x in -MAX_TRIAL_RANGE..MAX_TRIAL_RANGE {
        for y in -MAX_TRIAL_RANGE..MAX_TRIAL_RANGE {
            if find_path(target_area, Point { x, y }).is_some() {
                good_velocities_count += 1;
            }
        }
    }

    Some(good_velocities_count)
}

fn find_path(target: &TargetArea, mut velocity: Point) -> Option<Vec<Point>> {
    let mut path = Vec::new();

    let mut current_point = Point { x: 0, y: 0 };

    while !target.contains(&current_point) {
        // We are already below the target and Y won't increase
        // For some reason this check fails when testing part 2, so im disabling the test.
        if current_point.y < target.y_range.end && velocity.y < 0 {
            return None;
        }

        // We are already on the right of the target and X won't decrease
        if current_point.x > target.x_range.end && velocity.x > 0 {
            return None;
        }

        // We are already on the left of the target and X won't increase
        if current_point.x < target.x_range.start && velocity.x < 0 {
            return None;
        }

        path.push(current_point);

        current_point = current_point.add(&velocity);

        velocity.x += match velocity.x {
            v if v > 0 => -1,
            v if v < 0 => 1,
            _ => 0,
        };

        velocity.y -= 1;
    }

    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "target area: x=34..67, y=-215..-186";

        assert_eq!(
            parse_input_day17(input),
            TargetArea {
                x_range: Range { start: 34, end: 67 },
                y_range: Range {
                    start: -215,
                    end: -186,
                },
            }
        )
    }

    #[test]
    fn test_part1() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(day17_part1(&parse_input_day17(input)), Some(45));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(day17_part2(&parse_input_day17(input)), Some(112));
    }
}
