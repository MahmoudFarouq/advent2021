use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day6)]
fn parse_input_day6(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split_terminator(',').map(|l| l.parse()).collect()
}

#[aoc(day6, part1)]
fn day6_part1(input: &[i32]) -> Option<usize> {
    Some(solution(input, 80))
}

#[aoc(day6, part2)]
fn day6_part2(input: &[i32]) -> Option<usize> {
    Some(solution(input, 256))
}

fn solution(input: &[i32], days: usize) -> usize {
    let mut fish = [0; 9];

    for i in input {
        fish[*i as usize] += 1;
    }

    for _ in 0..days {
        let temp = fish[0];

        for i in 0..8 {
            fish[i] = fish[i + 1];
        }
        fish[8] = temp;

        fish[6] += fish[8];
    }

    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![3, 4, 3, 1, 2];
        assert_eq!(day6_part1(&input), Some(5934));
    }

    #[test]
    fn test_part2() {
        let input = vec![3, 4, 3, 1, 2];
        assert_eq!(day6_part2(&input), Some(26984457539));
    }
}
