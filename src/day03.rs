use aoc_runner_derive::{aoc, aoc_generator};

struct ParsedInput {
    entry_length: u32,
    data: Vec<usize>,
}

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> ParsedInput {
    let mut entry_length = 0;
    let data = input
        .lines()
        .map(|l| {
            entry_length = l.len();
            usize::from_str_radix(l, 2).unwrap()
        })
        .collect();

    ParsedInput {
        entry_length: entry_length as u32,
        data,
    }
}

struct BitsIterator {
    num: usize,
    mask: usize,
}

impl BitsIterator {
    fn new_with_length(num: usize, length: u32) -> Self {
        BitsIterator {
            num,
            mask: 1 << (length - 1),
        }
    }
}

impl Iterator for BitsIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mask {
            0 => None,
            _ => {
                let r = self.num & self.mask;
                self.mask >>= 1;
                Some(if r == 0 { 0 } else { 1 })
            }
        }
    }
}

#[aoc(day3, part1)]
fn day3_part1(parsed_input: &ParsedInput) -> Option<i32> {
    let record_len = parsed_input.entry_length;
    let input_len = parsed_input.data.len();

    let final_rates = parsed_input
        .data
        .iter()
        .fold(vec![0; record_len as usize], |mut counts, record| {
            BitsIterator::new_with_length(*record, record_len)
                .enumerate()
                .for_each(|(i, bit)| counts[i] += bit);
            counts
        })
        .iter()
        .fold((0, 0), |(b, e), count| {
            let flag = input_len >> 1 > *count;
            ((b << 1) | flag as i32, (e << 1) | !flag as i32)
        });

    Some(final_rates.0 * final_rates.1)
}

#[aoc(day3, part2)]
fn day3_part2(parsed_input: &ParsedInput) -> Option<i32> {
    let record_len = parsed_input.entry_length;

    let mut input = parsed_input.data.to_vec();
    input.sort_unstable();
    input.reverse();

    let oxy_selector = |ones_count: usize, zeroes_count: usize| ones_count >= zeroes_count;
    let oxy = magic_search(&input, record_len, oxy_selector);

    let scu_selector = |ones_count: usize, zeroes_count: usize| zeroes_count > ones_count;
    let scu = magic_search(&input, record_len, scu_selector);

    Some((oxy * scu) as i32)
}

fn magic_search(data: &[usize], len: u32, selector: fn(usize, usize) -> bool) -> usize {
    let mut bits_data = data
        .iter()
        .map(|r| BitsIterator::new_with_length(*r, len))
        .collect::<Vec<BitsIterator>>();

    smaller_magic_search(&mut bits_data, selector)
}

fn smaller_magic_search(
    bits_data: &mut [BitsIterator],
    selector: fn(usize, usize) -> bool,
) -> usize {
    if bits_data.len() == 1 {
        return bits_data[0].num;
    }

    let mut ones_count = 0;
    for item in bits_data.iter_mut() {
        if let Some(1) = item.next() {
            ones_count += 1;
        }
    }

    let zeroes_count = bits_data.len() - ones_count;

    if selector(ones_count, zeroes_count) {
        smaller_magic_search(&mut bits_data[..ones_count], selector)
    } else {
        smaller_magic_search(&mut bits_data[ones_count..], selector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_iterator_using_new_with_length() {
        let mut iterator = BitsIterator::new_with_length(5, 8);

        // TODO: Find a way to compare iterators or vectors.
        assert_eq!(Some(0), iterator.next());
        assert_eq!(Some(0), iterator.next());
        assert_eq!(Some(0), iterator.next());
        assert_eq!(Some(0), iterator.next());
        assert_eq!(Some(0), iterator.next());
        assert_eq!(Some(1), iterator.next());
        assert_eq!(Some(0), iterator.next());
        assert_eq!(Some(1), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn test_part1() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        assert_eq!(
            day3_part1(&ParsedInput {
                data: input,
                entry_length: 5,
            }),
            Some(198)
        );
    }

    #[test]
    fn test_part2() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        assert_eq!(
            day3_part2(&ParsedInput {
                data: input,
                entry_length: 5,
            }),
            Some(230)
        );
    }
}
