use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Vec<String> {
    input.lines().map(str::to_string).collect()
}

#[derive(Debug)]
struct Rates {
    beta: i32,
    epsilon: i32,
}

#[aoc(day3, part1)]
fn day3_part1(input: &[String]) -> Option<i32> {
    let record_len = input[0].len();
    let counts = vec![0; record_len];
    let initial_rates = Rates {
        beta: 0,
        epsilon: 0,
    };
    let input_len = input.len();

    let final_rates = input
        .iter()
        .fold(counts, |mut counts, record| {
            record
                .chars()
                .enumerate()
                .for_each(|(i, c)| counts[i] += c.to_digit(2).unwrap_or(0));
            counts
        })
        .iter()
        .fold(initial_rates, |mut moving_rate, count| {
            moving_rate.beta = (moving_rate.beta << 1) | (input_len > (count * 2) as usize) as i32;
            moving_rate.epsilon =
                (moving_rate.epsilon << 1) | (input_len <= (count * 2) as usize) as i32;

            moving_rate
        });

    println!("{:?}", final_rates);

    Some(final_rates.beta * final_rates.epsilon)
}

#[aoc(day3, part2)]
fn day3_part2(input: &[String]) -> Option<i32> {
    let record_len = input[0].len();

    let mut oxygen_records = input.to_owned();
    for i in 0..record_len {
        if oxygen_records.len() == 1 {
            break;
        }

        let mut ones = Vec::new();
        let mut zeroes = Vec::new();
        for record in oxygen_records {
            if record.get(i..i + 1).unwrap() == "1" {
                ones.push(record);
            } else {
                zeroes.push(record);
            }
        }

        if ones.len() > zeroes.len() || ones.len() == zeroes.len() {
            oxygen_records = ones
        } else {
            oxygen_records = zeroes
        }
    }

    let mut scrubber_records = input.to_owned();
    for i in 0..record_len {
        if scrubber_records.len() == 1 {
            break;
        }

        let mut ones = Vec::new();
        let mut zeroes = Vec::new();
        for record in scrubber_records {
            if record.get(i..i + 1).unwrap() == "1" {
                ones.push(record);
            } else {
                zeroes.push(record);
            }
        }

        if ones.len() > zeroes.len() || ones.len() == zeroes.len() {
            scrubber_records = zeroes
        } else {
            scrubber_records = ones
        }
    }

    let oxy = isize::from_str_radix(&oxygen_records[0], 2).unwrap();
    let scu = isize::from_str_radix(&scrubber_records[0], 2).unwrap();

    Some((oxy * scu) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        assert_eq!(day3_part1(&input), Some(198));
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        assert_eq!(day3_part2(&input), Some(230));
    }
}
