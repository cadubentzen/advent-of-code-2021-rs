#![feature(drain_filter)]

use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day04.txt");

fn main() {
    let (numbers_drawn, mut bingo) = parse_input(INPUT);

    let mut found_winner = false;
    for n in numbers_drawn {
        let mut winners = bingo.draw(n);
        if !winners.is_empty() {
            // Part 1
            if !found_winner {
                assert!(winners.len() == 1);
                let winner_board = winners.pop().unwrap();
                println!("Part 1");
                println!("Winning number: {}", n);
                let sum_unmarked: u16 = winner_board.unmarked().into_iter().map(|n| n as u16).sum();
                println!("Sum of unmarked numbers: {}", sum_unmarked);
                println!("Answer 1: {}\n", n as u16 * sum_unmarked);
                found_winner = true;
            }
            // Part 2
            if bingo.boards.is_empty() {
                assert!(winners.len() == 1);
                let winner_board = winners.pop().unwrap();
                println!("Part 2");
                println!("Last Winning number: {}", n);
                let sum_unmarked: u16 = winner_board.unmarked().into_iter().map(|n| n as u16).sum();
                println!("Sum of unmarked numbers: {}", sum_unmarked);
                println!("Answer 2: {}", n as u16 * sum_unmarked);
                break;
            }
        }
    }
}

const BINGO_SIZE: usize = 5;

#[derive(Debug, PartialEq, Clone)]
struct Board {
    numbers: [[u8; BINGO_SIZE]; BINGO_SIZE],
    rows: [HashSet<u8>; BINGO_SIZE],
    columns: [HashSet<u8>; BINGO_SIZE],
    marked: Vec<u8>,
}

impl Board {
    fn new(rows: Vec<[u8; BINGO_SIZE]>) -> Self {
        let numbers: [[u8; BINGO_SIZE]; BINGO_SIZE] = rows.clone().try_into().unwrap();

        let mut columns: [HashSet<u8>; BINGO_SIZE] = Default::default();
        (0..BINGO_SIZE).for_each(|row| {
            (0..BINGO_SIZE).for_each(|col| {
                columns[col].insert(numbers[row][col]);
            });
        });

        let rows = rows
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            numbers,
            rows,
            columns,
            marked: Vec::new(),
        }
    }

    fn builder() -> BoardBuilder {
        BoardBuilder { rows: Vec::new() }
    }

    fn unmarked(&self) -> HashSet<u8> {
        self.rows
            .iter()
            .cloned()
            .chain(self.columns.iter().cloned())
            .flatten()
            .collect()
    }

    fn mark(&mut self, num: u8) -> bool {
        let mut added = false;
        let mut has_won = false;
        for rc in self.rows.iter_mut().chain(self.columns.iter_mut()) {
            if rc.remove(&num) {
                if !added {
                    self.marked.push(num);
                    added = true;
                }
                if rc.is_empty() {
                    has_won = true;
                }
            }
        }
        has_won
    }
}

impl PartialEq<[[u8; BINGO_SIZE]; BINGO_SIZE]> for Board {
    fn eq(&self, other: &[[u8; BINGO_SIZE]; BINGO_SIZE]) -> bool {
        &self.numbers == other
    }
}

#[derive(Debug)] // TODO: use const generics to avoid using Vec
struct BoardBuilder {
    rows: Vec<[u8; BINGO_SIZE]>,
}

impl BoardBuilder {
    fn add_row(mut self, row: &str) -> Self {
        let row = row
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        self.rows.push(row);
        self
    }

    fn complete(self) -> Board {
        assert!(self.rows.len() == BINGO_SIZE);
        Board::new(self.rows)
    }
}

struct Bingo {
    boards: Vec<Board>,
}

impl Bingo {
    fn draw(&mut self, num: u8) -> Vec<Board> {
        self.boards.drain_filter(|b| b.mark(num)).collect()
    }
}

fn parse_input(input: &str) -> (Vec<u8>, Bingo) {
    let mut lines = input.lines();

    // First line contains the numbers drawn, comma separated
    let numbers_drawn: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|e| e.parse().unwrap())
        .collect();

    // Remaining lines will contain the bingo boards
    let mut boards = Vec::<Board>::new();

    let mut board = Board::builder();
    let mut counter = 0;
    for line in lines {
        if counter == 0 {
            counter += 1;
            continue;
        }
        board = board.add_row(line);
        if counter == BINGO_SIZE {
            counter = 0;
            boards.push(board.complete());
            board = Board::builder();
        } else {
            counter += 1;
        }
    }

    (numbers_drawn, Bingo { boards })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7
";

    #[test]
    fn example() {
        let (numbers_drawn, mut bingo) = parse_input(INPUT_EXAMPLE);

        assert_eq!(
            numbers_drawn,
            [
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );

        assert_eq!(
            bingo.boards,
            [
                [
                    [22, 13, 17, 11, 0],
                    [8, 2, 23, 4, 24],
                    [21, 9, 14, 16, 7],
                    [6, 10, 3, 18, 5],
                    [1, 12, 20, 15, 19]
                ],
                [
                    [3, 15, 0, 2, 22],
                    [9, 18, 13, 17, 5],
                    [19, 8, 7, 25, 23],
                    [20, 11, 10, 24, 4],
                    [14, 21, 16, 12, 6]
                ],
                [
                    [14, 21, 17, 24, 4],
                    [10, 16, 15, 9, 19],
                    [18, 8, 23, 26, 20],
                    [22, 11, 13, 6, 5],
                    [2, 0, 12, 3, 7]
                ]
            ]
        );

        // Part 1
        for n in [7, 4, 9, 5, 11] {
            assert!(bingo.draw(n).is_empty());
        }

        for board in &bingo.boards {
            assert_eq!(board.marked, [7, 4, 9, 5, 11]);
        }

        for n in [17, 23, 2, 0, 14, 21] {
            assert!(bingo.draw(n).is_empty());
        }
        let winning_number = 24;
        let winner_board = bingo.draw(winning_number).pop().unwrap();
        assert_eq!(
            winner_board,
            [
                [14, 21, 17, 24, 4],
                [10, 16, 15, 9, 19],
                [18, 8, 23, 26, 20],
                [22, 11, 13, 6, 5],
                [2, 0, 12, 3, 7]
            ]
        );

        assert_eq!(*winner_board.marked.last().unwrap(), winning_number);
        let sum_unmarked: u16 = winner_board.unmarked().into_iter().map(|n| n as u16).sum();
        assert_eq!(sum_unmarked, 188);
        assert_eq!(winning_number as u16 * sum_unmarked, 4512);

        // Part 2
        for n in [10, 16] {
            bingo.draw(n);
        }
        assert_eq!(bingo.boards.len(), 1);
        let last_number = 13;
        let last_winner = bingo.draw(last_number).pop().unwrap();
        assert_eq!(
            last_winner,
            [
                [3, 15, 0, 2, 22],
                [9, 18, 13, 17, 5],
                [19, 8, 7, 25, 23],
                [20, 11, 10, 24, 4],
                [14, 21, 16, 12, 6]
            ]
        );
        let sum_unmarked: u16 = last_winner.unmarked().into_iter().map(|n| n as u16).sum();
        assert_eq!(sum_unmarked, 148);
        assert_eq!(sum_unmarked * last_number as u16, 1924);
    }
}
