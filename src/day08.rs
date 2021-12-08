use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Decoder {
    dictionary: HashMap<String, usize>,
}

impl Decoder {
    fn new(signal_patterns: Vec<String>) -> Self {
        Decoder {
            dictionary: Self::decode_signal_patterns(signal_patterns),
        }
    }

    fn decode(&self, sequence: &str) -> Option<usize> {
        self.dictionary
            .get(&sequence.chars().sorted().collect::<String>())
            .cloned()
    }

    fn decode_signal_patterns(signal_patterns: Vec<String>) -> HashMap<String, usize> {
        let mut result = vec![String::new(); 10];

        let mut fives = Vec::new();
        let mut sixes = Vec::new();

        for p in signal_patterns {
            match p.len() {
                2 => result[1] = p,
                3 => result[7] = p,
                4 => result[4] = p,
                7 => result[8] = p,
                5 => fives.push(p),
                6 => sixes.push(p),
                _ => unreachable!(),
            }
        }

        for p in fives {
            if p.intersect(&result[4]).len() == 2 {
                result[2] = p
            } else if p.intersect(&result[1]).len() == 2 {
                result[3] = p
            } else {
                result[5] = p
            }
        }

        for p in sixes {
            if p.intersect(&result[1]).len() == 1 {
                result[6] = p
            } else if p.intersect(&result[4]).len() == 4 {
                result[9] = p
            } else {
                result[0] = p
            }
        }

        result
            .iter()
            .map(|p| p.chars().sorted().collect::<String>())
            .zip((0..result.len()).into_iter())
            .collect()
    }
}

trait Intersection<T> {
    fn intersect(&self, other: &T) -> T;
}

impl Intersection<String> for String {
    fn intersect(&self, other: &String) -> String {
        self.chars()
            .collect::<HashSet<_>>()
            .intersection(&other.chars().collect::<HashSet<_>>())
            .collect()
    }
}

#[derive(Debug)]
struct Entry {
    patterns: Vec<String>,
    output: Vec<String>,
}

#[aoc_generator(day8)]
fn parse_input_day8(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(" | ");

            Entry {
                patterns: parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(str::to_string)
                    .collect_vec(),
                output: parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(str::to_string)
                    .collect_vec(),
            }
        })
        .collect_vec()
}

#[aoc(day8, part1)]
fn day8_part1(input: &[Entry]) -> usize {
    let mut index = [0; 10];
    index[2] = 1;
    index[3] = 1;
    index[4] = 1;
    index[7] = 1;

    input
        .iter()
        .map(|e| e.output.clone())
        .flatten()
        .filter(|o| index[o.len()] == 1)
        .count()
}

#[aoc(day8, part2)]
fn day8_part2(input: &[Entry]) -> usize {
    input
        .iter()
        .map(|e| {
            let decoder = Decoder::new(e.patterns.to_vec());
            e.output
                .iter()
                .filter_map(|o| decoder.decode(o))
                .fold(0, |sum, o| sum * 10 + o)
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(day8_part1(&parse_input_day8(input)), 26);
    }

    #[test]
    fn test_part2() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(day8_part2(&parse_input_day8(input)), 61229);
    }
}

/*

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf

ab -> size 2 -> 1
dab -> size 3 -> 7
acedgfb -> size 7 -> 8
eafb -> size 4 -> 4



*/
