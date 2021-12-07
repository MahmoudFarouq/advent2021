use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day7)]
fn parse_input_day7(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(',').map(|l| l.parse()).collect()
}

#[aoc(day7, part1)]
fn day7_part1(input: &[i32]) -> i32 {
    let mut input = input.to_vec();

    input.sort_unstable();

    let point = input[input.len() / 2];
    input
        .iter()
        .fold(0, |sum, crap| sum + i32::abs(crap - point))
}

#[aoc(day7, part2)]
fn day7_part2(input: &[i32]) -> i32 {
    let mut best_fuel = i32::MAX;

    let input = input.to_vec();
    let min_test = input.iter().cloned().fold(i32::MAX, i32::min);
    let max_test = input.iter().cloned().fold(i32::MIN, i32::max);

    for i in min_test..max_test {
        best_fuel = best_fuel.min(input.iter().fold(0, |sum, crap| {
            sum + (i32::abs(crap - i) * (i32::abs(crap - i) + 1)) / 2
        }))
    }

    best_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(day7_part1(&input), 37);
    }

    #[test]
    fn test_part2() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(day7_part2(&input), 168);
    }
}
