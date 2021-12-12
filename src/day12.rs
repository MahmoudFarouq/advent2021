use std::collections::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use crate::day12::Cave::{End, Small, Start};

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

type Path = Vec<Cave>;

#[derive(Default)]
struct AdjacencyList {
    map: HashMap<Cave, Vec<Cave>>,
}

impl AdjacencyList {
    fn new() -> Self {
        Default::default()
    }

    fn find_paths(&self, from: Cave, to: Cave, is_good_path: fn(&Path) -> bool) -> Vec<Path> {
        let mut paths = Vec::new();

        let mut stack = vec![vec![from.clone()]];

        while !stack.is_empty() {
            let current = stack.pop().unwrap();

            let last_step = current.last().unwrap().clone();

            if last_step == to {
                paths.push(current);

                continue;
            }

            if !is_good_path(&current) {
                continue;
            }

            if last_step == from && current.len() > 1 {
                continue;
            }

            // stack.extend(self.map.get(&last_step).unwrap_or(&Vec::new()).clone());
            if let Some(connections) = self.map.get(&last_step) {
                for connection in connections.clone() {
                    let mut clone = current.clone();
                    clone.push(connection);

                    stack.push(clone);
                }
            }
        }

        paths
    }
}

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        match s {
            "start" => Cave::Start,
            "end" => Cave::End,
            s if s.to_lowercase() == s => Cave::Small(s.to_string()),
            s if s.to_uppercase() == s => Cave::Big(s.to_string()),
            _ => unreachable!()
        }
    }
}

impl FromIterator<(Cave, Cave)> for AdjacencyList {
    fn from_iter<T: IntoIterator<Item=(Cave, Cave)>>(iter: T) -> Self {
        let mut list = AdjacencyList::new();

        iter
            .into_iter()
            .for_each(|(from, to): (Cave, Cave)| {
                list.map.entry(from.clone()).or_default().push(to.clone());
                list.map.entry(to).or_default().push(from);
            });

        list
    }
}

#[aoc_generator(day12)]
fn parse_input_day12(input: &str) -> AdjacencyList {
    input.lines().map(|l| l.split('-').map(Cave::from).collect_tuple::<(Cave, Cave)>().unwrap()).collect()
}

#[aoc(day12, part1)]
fn day12_part1(input: &AdjacencyList) -> Option<usize> {
    let is_good_path = |p: &Path| -> bool {
        let last_step = p.last().unwrap().clone();

        if let Small(_) = last_step {
            p.iter().filter(|&c| *c == last_step).count() == 1
        } else {
            true
        }
    };

    Some(input.find_paths(Start, End, is_good_path).len())
}

#[aoc(day12, part2)]
fn day12_part2(input: &AdjacencyList) -> Option<usize> {
    let is_good_path = |p: &Path| -> bool {
        let all_smalls = p
            .iter()
            .filter_map(|c| if let Small(s) = c { Some(s) } else { None })
            .collect_vec();

        let set = all_smalls.iter().collect::<HashSet<_>>();

        set.len() == all_smalls.len() || set.len() + 1 == all_smalls.len()
    };

    Some(input.find_paths(Start, End, is_good_path).len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(day12_part1(&parse_input_day12(input)), Some(10));
    }

    #[test]
    fn test_part2() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(day12_part2(&parse_input_day12(input)), Some(36));
    }
}
