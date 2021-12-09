use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Board<T: Clone + Copy> {
    width: usize,
    height: usize,
    inner: Vec<T>,
}

impl<T: Copy> Board<T> {
    fn new_from_matrix(mat: Vec<Vec<T>>) -> Self {
        Self {
            width: mat[0].len(),
            height: mat.len(),
            inner: mat.iter().cloned().flatten().collect_vec(),
        }
    }

    fn adjacent_points(&self, index: Point) -> Vec<Point> {
        let mut result = Vec::with_capacity(4);

        if index.x < self.width - 1 {
            result.push(Point {
                x: index.x + 1,
                y: index.y,
            })
        }

        if index.y < self.height - 1 {
            result.push(Point {
                x: index.x,
                y: index.y + 1,
            })
        }

        if index.x > 0 {
            result.push(Point {
                x: index.x - 1,
                y: index.y,
            })
        }

        if index.y > 0 {
            result.push(Point {
                x: index.x,
                y: index.y - 1,
            })
        }

        result
    }
}

impl<T: Copy> Index<Point> for Board<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        let point_to_index = |p: Point| p.y * self.width + p.x;
        &self.inner[point_to_index(index)]
    }
}

impl<T: Copy> IndexMut<Point> for Board<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let point_to_index = |p: Point| p.y * self.width + p.x;
        self.inner.get_mut(point_to_index(index)).unwrap()
    }
}

struct BoardIterator<T: Clone + Copy> {
    board: Board<T>,
    last_index: usize,
}

impl<T: Copy> BoardIterator<T> {
    fn new(board: Board<T>) -> Self {
        BoardIterator {
            board,
            last_index: 0,
        }
    }
}

impl<T: Copy> Iterator for BoardIterator<T> {
    type Item = (Point, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_index >= self.board.height * self.board.width {
            None
        } else {
            let index_to_point = |i: usize| Point {
                x: i % self.board.width,
                y: i / self.board.width,
            };

            let p = index_to_point(self.last_index);

            self.last_index += 1;

            Some((p, self.board[p]))
        }
    }
}

impl<T: Copy> IntoIterator for Board<T> {
    type Item = (Point, T);
    type IntoIter = BoardIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator::new(self)
    }
}

struct UnionFind {
    mask: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            mask: (0..size).into_iter().collect_vec(),
        }
    }

    fn union(&mut self, p1: usize, p2: usize) {
        let p1 = self.find(p1);
        let p2 = self.find(p2);
        self.mask[p1] = self.mask[p2];

        for p in 0..self.mask.len() {
            if self.mask[p] == p1 {
                self.mask[p] = self.mask[p2];
            }
        }
    }

    fn find(&mut self, mut p: usize) -> usize {
        while p != self.mask[p] {
            p = self.mask[p];
        }

        p
    }
}

#[aoc_generator(day9)]
fn parse_input_day9(input: &str) -> Board<usize> {
    Board::new_from_matrix(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect_vec()
            })
            .collect_vec(),
    )
}

#[aoc(day9, part1)]
fn day9_part1(input: &Board<usize>) -> usize {
    input
        .clone()
        .into_iter()
        .filter_map(|(p, _)| {
            if input
                .adjacent_points(p)
                .into_iter()
                .all(|adj| input[p] < input[adj])
            {
                Some(p)
            } else {
                None
            }
        })
        .map(|p| 1 + input[p])
        .sum()
}

#[aoc(day9, part2)]
fn day9_part2(input: &Board<usize>) -> usize {
    let point_to_index = |p: Point| p.y * input.width + p.x;

    let mut uf = UnionFind::new(input.width * input.height);

    input
        .clone()
        .into_iter()
        .filter_map(|(p, _)| {
            let mut lowest_of_adjacent = p;

            for adj in input.adjacent_points(p) {
                if input[adj] < input[lowest_of_adjacent] {
                    lowest_of_adjacent = adj;
                }
            }

            if input[p] == 9 {
                return None;
            }

            if p == lowest_of_adjacent {
                return None;
            }

            Some((p, lowest_of_adjacent))
        })
        .for_each(|(p, adj)| uf.union(point_to_index(p), point_to_index(adj)));

    let mut counts: HashMap<usize, usize> = HashMap::new();
    uf.mask
        .iter()
        .for_each(|p| *counts.entry(*p).or_default() += 1);

    counts.values().into_iter().sorted().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(day9_part1(&parse_input_day9(input)), 15);
    }

    #[test]
    fn test_part2() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(day9_part2(&parse_input_day9(input)), 1134);
    }
}
