use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct InsertionRule {
    sequence: (char, char),
    insert: char,
}

#[derive(Debug)]
struct Input {
    polymer_template: Vec<char>,
    pair_insertion_rules: Vec<InsertionRule>,
}

#[aoc_generator(day14)]
fn parse_input_day14(input: &str) -> Input {
    let parts = input.split_once("\n\n").unwrap();

    Input {
        polymer_template: parts.0.chars().collect(),
        pair_insertion_rules: parts
            .1
            .lines()
            .map(|l| {
                let parts = l.split_once(" -> ").unwrap();

                InsertionRule {
                    sequence: parts.0.chars().collect_tuple().unwrap(),
                    insert: parts.1.chars().next().unwrap(),
                }
            })
            .collect(),
    }
}

#[aoc(day14, part1)]
fn day14_part1(input: &Input) -> Option<usize> {
    solve(input, 10)
}

#[aoc(day14, part2)]
fn day14_part2(input: &Input) -> Option<usize> {
    solve(input, 40)
}

fn solve(input: &Input, iterations: usize) -> Option<usize> {
    let mut sequence = input
        .polymer_template
        .iter()
        .cloned()
        .tuple_windows::<(char, char)>()
        .fold(
            HashMap::<(char, char), usize>::new(),
            |mut counter, elem| {
                *counter.entry(elem).or_default() += 1;
                counter
            },
        );

    let mut char_counts =
        input
            .polymer_template
            .iter()
            .fold(HashMap::<char, usize>::new(), |mut counter, elem| {
                *counter.entry(*elem).or_default() += 1;
                counter
            });

    for _ in 1..=iterations {
        let mut new_sequence = sequence.clone();

        for rule in input.pair_insertion_rules.iter() {
            if let Some(n) = sequence.get(&rule.sequence) {
                *new_sequence.entry(rule.sequence).or_default() -= n;
                *new_sequence
                    .entry((rule.sequence.0, rule.insert))
                    .or_default() += n;
                *new_sequence
                    .entry((rule.insert, rule.sequence.1))
                    .or_default() += n;
                *char_counts.entry(rule.insert).or_default() += n;
            }
        }

        sequence = new_sequence;
    }

    let sorted_counts = char_counts.values().cloned().sorted().collect_vec();

    Some(sorted_counts[sorted_counts.len() - 1] - sorted_counts[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(day14_part1(&parse_input_day14(input)), Some(1588));
    }

    #[test]
    fn test_part2() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(day14_part2(&parse_input_day14(input)), Some(2188189693529));
    }
}
