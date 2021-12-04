use crate::day04::BoardTile::{Marked, Unmarked};
use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum BoardTile {
    Unmarked(u32),
    Marked,
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<BoardTile>>,
}

impl Board {
    fn mark(&mut self, value: u32) {
        for i in 0..5 {
            for j in 0..5 {
                if Unmarked(value) == self.tiles[i][j] {
                    self.tiles[i][j] = Marked
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        'check_rows: for i in 0..5 {
            for j in 0..5 {
                if let Unmarked(_) = self.tiles[i][j] {
                    continue 'check_rows;
                }
            }

            return true;
        }

        'check_columns: for i in 0..5 {
            for j in 0..5 {
                if let Unmarked(_) = self.tiles[j][i] {
                    continue 'check_columns;
                }
            }

            return true;
        }

        false
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;

        for i in 0..5 {
            for j in 0..5 {
                if let Unmarked(n) = self.tiles[i][j] {
                    sum += n
                }
            }
        }

        sum
    }
}

impl From<Vec<&str>> for Board {
    fn from(b: Vec<&str>) -> Self {
        if b.len() != 5 {
            panic!("board must have 5 columns")
        }

        let mut tiles = vec![vec![Marked; 5]; 5];

        b.iter().enumerate().for_each(|(i, &row)| {
            tiles[i] = row
                .split_whitespace()
                .map(|s| Unmarked(s.parse().unwrap()))
                .collect_vec();
        });

        Board { tiles }
    }
}

impl Board {}

struct Input {
    sequence: Vec<u32>,
    boards: Vec<Board>,
}

fn parse_input(input: &str) -> Input {
    let mut chunks = input.split_terminator("\n\n");

    let mut input = Input {
        sequence: chunks
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect(),
        boards: chunks
            .into_iter()
            .map(|b| Board::from(b.lines().collect_vec()))
            .collect_vec(),
    };

    input.sequence.reverse();

    input
}

#[aoc(day4, part1)]
fn day4_part1(input: &str) -> Option<u32> {
    let mut input = parse_input(input);

    loop {
        let number = input.sequence.pop().unwrap();
        for board in input.boards.iter_mut() {
            board.mark(number);

            if board.has_won() {
                return Some(board.sum_unmarked() * number);
            }
        }
    }
}

#[aoc(day4, part2)]
fn day4_part2(input: &str) -> Option<u32> {
    let mut input = parse_input(input);

    let mut last_winning_score = 0;
    let mut already_won_boards = Vec::with_capacity(input.boards.len());

    while !input.sequence.is_empty() {
        let number = input.sequence.pop().unwrap();

        for (i, board) in input.boards.iter_mut().enumerate() {
            board.mark(number);

            if already_won_boards.iter().all(|test| *test != i) && board.has_won() {
                last_winning_score = board.sum_unmarked() * number;

                already_won_boards.push(i);
            }
        }
    }

    Some(last_winning_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_day4() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let input = parse_input(&input);

        let mut sequence = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        sequence.reverse();

        assert_eq!(input.sequence, sequence);
        assert_eq!(
            input.boards.get(2).unwrap().tiles,
            vec![
                vec![
                    Unmarked(14),
                    Unmarked(21),
                    Unmarked(17),
                    Unmarked(24),
                    Unmarked(4),
                ],
                vec![
                    Unmarked(10),
                    Unmarked(16),
                    Unmarked(15),
                    Unmarked(9),
                    Unmarked(19),
                ],
                vec![
                    Unmarked(18),
                    Unmarked(8),
                    Unmarked(23),
                    Unmarked(26),
                    Unmarked(20),
                ],
                vec![
                    Unmarked(22),
                    Unmarked(11),
                    Unmarked(13),
                    Unmarked(6),
                    Unmarked(5),
                ],
                vec![
                    Unmarked(2),
                    Unmarked(0),
                    Unmarked(12),
                    Unmarked(3),
                    Unmarked(7),
                ],
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(day4_part1(&input), Some(4512));
    }

    #[test]
    fn test_part2() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(day4_part2(&input), Some(1924));
    }
}
