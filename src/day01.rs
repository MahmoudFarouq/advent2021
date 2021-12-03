use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

/// 1696
#[aoc(day1, part1)]
fn day1_part1(input: &[i32]) -> Option<i32> {
    input
        .windows(2)
        .fold(0, |c, w| if w[1] > w[0] { c + 1 } else { c })
        .into()
}

/// 1737
#[aoc(day1, part2)]
fn day1_part2(input: &[i32]) -> Option<i32> {
    let mut count = 0;

    let mut windows = input.windows(3).into_iter();

    let mut cursor = windows.next().map(|w| w.iter().sum::<i32>()).unwrap();

    for window in windows {
        let new_sum = window.iter().sum::<i32>();

        if new_sum > cursor {
            count += 1
        }

        cursor = new_sum
    }

    count.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(day1_part1(&input), Some(7));
    }

    #[test]
    fn test_part2() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(day1_part2(&input), Some(5));
    }
}
