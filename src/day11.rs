use crate::day11::Octopus::{Charging, Flashing};
use ansi_term::Colour::Red;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Octopus {
    Flashing,
    Charging(u32),
}

impl From<char> for Octopus {
    fn from(c: char) -> Self {
        match c.to_digit(10) {
            Some(0) => Flashing,
            Some(n) => Charging(n),
            None => unreachable!(),
        }
    }
}

struct DumboOctopuses {
    width: usize,
    height: usize,
    octopuses: Vec<Octopus>,
}

impl DumboOctopuses {
    fn step(&mut self) {
        let mut should_flash = Vec::new();

        for (i, o) in self.octopuses.iter_mut().enumerate() {
            match o {
                Charging(9) => should_flash.push(i),
                Charging(n) => *o = Charging(*n + 1),
                Flashing => *o = Charging(1),
            }
        }

        while !should_flash.is_empty() {
            let i = should_flash.pop().unwrap();

            // Don't flash twice already!.
            if let Flashing = self.octopuses[i] {
                continue;
            }

            self.octopuses[i] = Flashing;

            let adjacent = self.get_adjacent(i);
            for adj in adjacent.into_iter() {
                if let Some(o) = self.octopuses.get_mut(adj) {
                    match o {
                        Charging(9) => should_flash.push(adj),
                        Charging(n) => *o = Charging(*n + 1),
                        Flashing => continue,
                    }
                }
            }
        }
    }

    fn get_adjacent(&self, i: usize) -> Vec<usize> {
        let point_to_index = |p: (usize, usize)| p.1 * self.width + p.0;
        let index_to_point = |i: usize| (i % self.width, i / self.height);

        let mut result = Vec::with_capacity(8);

        let i = index_to_point(i);

        // Left
        if i.0 > 0 {
            result.push(point_to_index((i.0 - 1, i.1)))
        }

        // Right
        if i.0 < self.width - 1 {
            result.push(point_to_index((i.0 + 1, i.1)))
        }

        // Up
        if i.1 > 0 {
            result.push(point_to_index((i.0, i.1 - 1)))
        }

        // Down
        if i.1 < self.height - 1 {
            result.push(point_to_index((i.0, i.1 + 1)))
        }

        // TopLeft
        if i.0 > 0 && i.1 > 0 {
            result.push(point_to_index((i.0 - 1, i.1 - 1)))
        }

        // TopRight
        if i.0 < self.width - 1 && i.1 > 0 {
            result.push(point_to_index((i.0 + 1, i.1 - 1)))
        }

        // BottomLeft
        if i.0 > 0 && i.1 < self.height - 1 {
            result.push(point_to_index((i.0 - 1, i.1 + 1)))
        }

        // BottomRight
        if i.0 < self.width - 1 && i.1 < self.height - 1 {
            result.push(point_to_index((i.0 + 1, i.1 + 1)))
        }

        result
    }

    fn count_flashed(&self) -> usize {
        self.octopuses.iter().filter(|&o| o == &Flashing).count()
    }
}

impl<T: IntoIterator<Item = char>> From<T> for DumboOctopuses {
    fn from(chain: T) -> Self {
        let octopuses = chain.into_iter().map(Octopus::from).collect_vec();

        let length = f32::sqrt(octopuses.len() as f32) as usize;

        DumboOctopuses {
            octopuses,
            width: length,
            height: length,
        }
    }
}

impl Debug for DumboOctopuses {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let side = f32::sqrt(self.octopuses.len() as f32) as usize;

        let mut builder = String::new();

        for i in 0..side {
            for j in 0..side {
                let part = match self.octopuses[i * side + j] {
                    Flashing => format!("{}", Red.paint(format!("{}", 0))),
                    Charging(n) => format!("{:?}", n),
                };

                builder += &part;
            }

            builder += "\n";
        }

        write!(f, "{}", builder)
    }
}

fn parse_input_day11(input: &str) -> DumboOctopuses {
    DumboOctopuses::from(input.lines().map(|c| c.chars().collect_vec()).flatten())
}

#[aoc(day11, part1)]
fn day11_part1(input: &str) -> usize {
    let mut dumbo = parse_input_day11(input);

    let mut sum = 0;

    for _ in 0..100 {
        dumbo.step();
        sum += dumbo.count_flashed();

        println!("{:?}", dumbo);
    }

    sum
}

#[aoc(day11, part2)]
fn day11_part2(input: &str) -> Option<usize> {
    let mut dumbo = parse_input_day11(input);

    for i in 1.. {
        dumbo.step();

        if dumbo.count_flashed() == 100 {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(day11_part1(&input), 1656);
    }

    #[test]
    fn test_part2() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(day11_part2(&input), Some(195));
    }
}
