use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&self, diff: &Point) -> Point {
        Point {
            x: self.x + diff.x,
            y: self.y + diff.y,
        }
    }
}

#[derive(Debug)]
struct Board<T> {
    inner: Vec<Vec<T>>,
}

impl<T: Copy> Board<T> {
    fn adjacent_points(&self, index: Point) -> Vec<Point> {
        const DIFFS: [Point; 4] = [
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: 0, y: -1 },
        ];

        let mut result = Vec::with_capacity(4);

        for diff in DIFFS.iter() {
            let new_point = index.add(diff);

            if new_point.x < 0 || new_point.x >= self.inner[0].len() as i32 {
                continue;
            }
            if new_point.y < 0 || new_point.y >= self.inner.len() as i32 {
                continue;
            }

            result.push(new_point)
        }

        result
    }
}

impl<T> Index<Point> for Board<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        self.inner
            .get(index.y as usize)
            .unwrap()
            .get(index.x as usize)
            .unwrap()
    }
}

impl<T> IndexMut<Point> for Board<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.inner
            .get_mut(index.y as usize)
            .unwrap()
            .get_mut(index.x as usize)
            .unwrap()
    }
}

#[aoc_generator(day9)]
fn parse_input_day9(input: &str) -> Board<usize> {
    Board {
        inner: input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect_vec()
            })
            .collect_vec(),
    }
}

#[derive(Debug)]
struct UnionFind2D<'a> {
    board: &'a Board<usize>,
    mask: Board<Point>,
}

impl<'a> UnionFind2D<'a> {
    fn new(board: &'a Board<usize>) -> Self {
        UnionFind2D {
            board,
            mask: Self::mask_from_board(board),
        }
    }

    fn mask_from_board(board: &Board<usize>) -> Board<Point> {
        let mut mask = Vec::new();

        for y in 0..board.inner.len() {
            let mut row = Vec::new();

            for x in 0..board.inner[0].len() {
                let p = Point {
                    x: x as i32,
                    y: y as i32,
                };
                row.push(p);
            }

            mask.push(row);
        }

        Board { inner: mask }
    }

    fn union(&mut self, p1: Point, p2: Point) {
        self.mask[p1] = self.mask[p2];

        // All that connects to p1 should now connect to p2
        // It's stupid to loop over all the matrix, this should be improved.
        for y in 0..self.mask.inner.len() {
            for x in 0..self.mask.inner[0].len() {
                let p = Point {
                    x: x as i32,
                    y: y as i32,
                };

                if self.mask[p] == p1 {
                    self.mask[p] = self.mask[p2];
                }
            }
        }
    }
}

#[aoc(day9, part1)]
fn day9_part1(input: &Board<usize>) -> usize {
    let mut low_points = Vec::new();

    // TODO: Should implement iterator over cells maybe? with enumerate?
    for y in 0..input.inner.len() {
        'check: for x in 0..input.inner[0].len() {
            let p = Point {
                x: x as i32,
                y: y as i32,
            };

            let adjacent = input.adjacent_points(p);

            for adj in adjacent {
                if input[p] >= input[adj] {
                    continue 'check;
                }
            }

            low_points.push(p);
        }
    }

    low_points.iter().map(|p| 1 + input[*p]).sum()
}

#[aoc(day9, part2)]
fn day9_part2(input: &Board<usize>) -> usize {
    let mut uf = UnionFind2D::new(input);

    for y in 0..input.inner.len() {
        for x in 0..input.inner[0].len() {
            let p = Point {
                x: x as i32,
                y: y as i32,
            };

            let adjacent = input.adjacent_points(p);

            let mut lowest_of_adjacent = p;

            for adj in adjacent {
                if input[adj] < input[lowest_of_adjacent] {
                    lowest_of_adjacent = adj;
                }
            }

            if input[p] == 9 {
                continue;
            }

            if p == lowest_of_adjacent {
                continue;
            }

            uf.union(p, lowest_of_adjacent);
        }
    }

    let mut lowest_points = Vec::new();

    for y in 0..input.inner.len() {
        for x in 0..input.inner[0].len() {
            let p = Point {
                x: x as i32,
                y: y as i32,
            };

            if uf.mask[p] == p && input[p] != 9 {
                lowest_points.push(p);
            }
        }
    }

    let mut counts: HashMap<Point, usize> = HashMap::new();
    for y in 0..input.inner.len() {
        for x in 0..input.inner[0].len() {
            let p = Point {
                x: x as i32,
                y: y as i32,
            };

            *counts.entry(uf.mask[p]).or_default() += 1;
        }
    }

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
