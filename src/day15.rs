use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
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
    inner: Vec<Vec<T>>,
}

impl<T: Copy> Board<T> {
    fn new(mat: Vec<Vec<T>>) -> Self {
        Self {
            width: mat[0].len(),
            height: mat.len(),
            inner: mat,
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
        &self.inner[index.y][index.x]
    }
}

impl<T: Copy> IndexMut<Point> for Board<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.inner
            .get_mut(index.y)
            .unwrap()
            .get_mut(index.x)
            .unwrap()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Composite {
    point: Point,
    cost: usize,
}

impl PartialOrd for Composite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Composite {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

struct PathFinder<'a> {
    graph: &'a Board<usize>,
}

impl<'a> PathFinder<'a> {
    fn new(graph: &'a Board<usize>) -> Self {
        Self { graph }
    }

    fn find_path(&self) -> Option<usize> {
        let start_point = Point { x: 0, y: 0 };
        let end_point = Point {
            x: self.graph.width - 1,
            y: self.graph.height - 1,
        };

        let mut dist = Board::new(vec![vec![usize::MAX; self.graph.width]; self.graph.height]);
        dist[start_point] = self.graph[start_point];

        let mut queue = BinaryHeap::new();
        queue.push(Composite {
            point: start_point,
            cost: self.graph[start_point],
        });

        while let Some(Composite { point, cost }) = queue.pop() {
            if point == end_point {
                return Some(cost - dist[start_point]);
            }

            if cost > dist[point] {
                continue;
            }

            for adj in self.graph.adjacent_points(point).iter() {
                let next = Composite {
                    point: *adj,
                    cost: cost + self.graph[*adj],
                };

                if next.cost < dist[*adj] {
                    dist[*adj] = next.cost;
                    queue.push(next);
                }
            }
        }

        None
    }
}

#[aoc_generator(day15)]
fn parse_input_day15(input: &str) -> Board<usize> {
    Board::new(
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

#[aoc(day15, part1)]
fn day15_part1(input: &Board<usize>) -> Option<usize> {
    PathFinder::new(input).find_path()
}

#[aoc(day15, part2)]
fn day15_part2(input: &Board<usize>) -> Option<usize> {
    let tile = input.inner.clone();

    let mut big_tile = vec![vec![0; tile[0].len() * 5]; tile.len() * 5];

    for y in 0..big_tile.len() {
        for x in 0..big_tile[0].len() {
            let j = x / tile[0].len() + y / tile.len();

            big_tile[x][y] = tile[x % tile[0].len()][y % tile.len()] + j;
            if big_tile[x][y] > 9 {
                big_tile[x][y] %= 9;
            }
        }
    }

    PathFinder::new(&Board::new(big_tile)).find_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!(day15_part1(&parse_input_day15(input)), Some(40));
    }

    #[test]
    fn test_part2() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!(day15_part2(&parse_input_day15(input)), Some(315));
    }
}
