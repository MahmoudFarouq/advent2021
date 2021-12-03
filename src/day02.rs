use crate::day02::Directions::{Down, Forward, Up};
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[derive(Copy, Clone, Debug)]
struct Location {
    x: i32,
    y: i32,
    aim: i32,
}

#[derive(Copy, Clone, Debug)]
enum Directions<T> {
    Forward(T),
    Down(T),
    Up(T),
}

impl From<&str> for Directions<i32> {
    fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();

        match parts.next() {
            Some("forward") => Forward(parts.next().unwrap().parse().unwrap()),
            Some("down") => Down(parts.next().unwrap().parse().unwrap()),
            Some("up") => Up(parts.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Result<Vec<Directions<i32>>, ParseIntError> {
    Ok(input
        .lines()
        .map(Directions::from)
        .collect::<Vec<Directions<i32>>>())
}

#[aoc(day2, part1)]
fn day2_part1(input: &[Directions<i32>]) -> Option<i32> {
    let initial_location = Location { x: 0, y: 0, aim: 0 };

    let final_location = input
        .iter()
        .fold(initial_location, |mut current_location, step| {
            match step {
                Forward(n) => current_location.x += n,
                Down(n) => current_location.y += n,
                Up(n) => current_location.y -= n,
            }

            current_location
        });

    Some(final_location.x * final_location.y)
}

#[aoc(day2, part2)]
fn day2_part2(input: &[Directions<i32>]) -> Option<i32> {
    let initial_location = Location { x: 0, y: 0, aim: 0 };

    let final_location = input
        .iter()
        .fold(initial_location, |mut current_location, step| {
            match step {
                Forward(n) => {
                    current_location.x += n;
                    current_location.y += current_location.aim * n;
                }
                Down(n) => current_location.aim += n,
                Up(n) => current_location.aim -= n,
            }

            current_location
        });

    Some(final_location.x * final_location.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        // assert_eq!(parse_input_day2(&input).unwrap(), vec![Forward(5 as i32), Down(5), Forward(8), Up(3), Down(8), Forward(2)]);
        println!("{:?}", parse_input_day2(&input).unwrap());
    }

    #[test]
    fn test_part1() {
        let input = vec![
            Forward(5 as i32),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2),
        ];
        assert_eq!(day2_part1(&input), Some(150))
    }

    #[test]
    fn test_part2() {
        let input = vec![
            Forward(5 as i32),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2),
        ];
        assert_eq!(day2_part2(&input), Some(900))
    }
}
