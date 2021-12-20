use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use crate::day20::Pixel::Dark;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Pixel {
    Dark,
    Light,
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Pixel::Dark => '.',
            Pixel::Light => '#'
        })
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel::Dark
    }
}


impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '#' => Pixel::Light,
            '.' => Pixel::Dark,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone)]
struct Board<T: Clone + Copy> {
    inner: Vec<Vec<T>>,
}

impl<T: Debug + Copy + Clone + Default> Debug for Board<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.inner.len() {
            for y in 0..self.inner[0].len() {
                write!(f, "{:?}", &self.index(Point { x: x as isize, y: y as isize }))?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}

impl<T: Copy + Default> Board<T> {
    fn new(mat: Vec<Vec<T>>) -> Self {
        Self { inner: mat }
    }

    fn convolution(&self, index: Point) -> Vec<Point> {
        vec![
            Point { x: index.x - 1, y: index.y - 1 },
            Point { x: index.x, y: index.y - 1 },
            Point { x: index.x + 1, y: index.y - 1 },
            Point { x: index.x - 1, y: index.y },
            Point { x: index.x, y: index.y },
            Point { x: index.x + 1, y: index.y },
            Point { x: index.x - 1, y: index.y + 1 },
            Point { x: index.x, y: index.y + 1 },
            Point { x: index.x + 1, y: index.y + 1 },
        ]
    }
}

impl<T: Copy> Index<Point> for Board<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        if index.x < 0 || index.y < 0 || index.x >= self.inner.len() as isize || index.y >= self.inner.len() as isize {
            return &self[Point{ x: 0, y: 0 }]
        }

        &self.inner[index.y as usize][index.x as usize]
    }
}

impl<T: Copy> IndexMut<Point> for Board<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        if index.x < 0 || index.y < 0 || index.x >= self.inner.len() as isize || index.y >= self.inner.len() as isize {
            return &mut self[Point{ x: 0, y: 0 }]
        }

        self.inner
            .get_mut(index.y as usize)
            .unwrap()
            .get_mut(index.x as usize)
            .unwrap()
    }
}

struct ParsedInput {
    algorithm: Vec<Pixel>,
    image: Board<Pixel>,
}

fn parse_input_day20(input: &str) -> Option<ParsedInput> {
    let mut parts = input.split("\n\n");

    let algorithm = parts.next().unwrap().chars().map(Pixel::from).collect();

    let mut map: Vec<Vec<Pixel>> = parts.next().unwrap().lines().map(|row| {
        let mut pad_left = vec![Dark; 50];

        pad_left.extend(row.chars().map(Pixel::from).chain(vec![Dark; 50]));

        pad_left
    }).collect();

    let len = map[0].len();

    for _ in 0..50 {
        map.insert(0, vec![Dark; len]);
        map.push(vec![Dark; len]);
    }

    Some(ParsedInput {
        algorithm,
        image: Board::new(map),
    })
}

#[aoc(day20, part1)]
fn day20_part1(input: &str) -> Option<usize> {
    enhance(input, 2)
}

#[aoc(day20, part2)]
fn day20_part2(input: &str) -> Option<usize> {
    enhance(input, 50)
}


fn enhance(input: &str, times: usize) -> Option<usize> {
    let ParsedInput { algorithm, mut image } = parse_input_day20(input).unwrap();

    for _ in 0..times {
        let mut new_image = image.clone();

        for (x, row) in image.inner.iter().enumerate() {
            for (y, p) in row.iter().enumerate() {
                let p = Point { x:x as isize, y: y as isize };

                let binary = image.convolution(p).iter().map(|p| {
                    match image[*p] {
                        Pixel::Light => '1',
                        Pixel::Dark => {
                            match new_image[*p] {
                                Pixel::Light => {}
                                Pixel::Dark => { new_image[*p] = Pixel::Dark }
                            };
                            '0'
                        }
                    }
                }).join("");

                new_image[p] = algorithm[usize::from_str_radix(binary.as_str(), 2).unwrap()]
            }
        }

        image = new_image;
    }

    let sum = image.inner.iter().fold(0, |cum, row| cum + row.iter().filter(|&v| v == &Pixel::Light).count());

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        assert_eq!(day20_part1(input), Some(35));
    }

    #[test]
    fn test_part2() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        assert_eq!(day20_part2(input), Some(3351));
    }
}
