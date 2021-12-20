use core::panic;
use std::{collections::HashSet, fs::read_to_string};

struct Board {
    rows: Vec<Vec<i32>>,
    cols: Vec<Vec<i32>>,
}

impl Board {
    fn new(rows: Vec<Vec<i32>>) -> Board {
        let col_length = rows.len();
        let num_cols = rows[0].len();

        let cols = (0..num_cols)
            .map(|x| (0..col_length).map(|y| rows[y][x]).collect())
            .collect();

        Board { rows, cols }
    }

    fn did_win(&self, drawn_numbers: &[i32]) -> bool {
        for row in &self.rows {
            if row.iter().all(|&x| drawn_numbers.contains(&x)) {
                return true;
            }
        }

        for col in &self.cols {
            if col.iter().all(|&x| drawn_numbers.contains(&x)) {
                return true;
            }
        }

        false
    }

    fn score(&self, drawn_numbers: &[i32]) -> i32 {
        let drawn_number_set = drawn_numbers.iter().collect::<HashSet<_>>();
        let board_numbers = self
            .rows
            .iter()
            .flatten()
            .collect::<HashSet<_>>()
            .difference(&drawn_number_set)
            .map(|a| **a)
            .collect::<Vec<_>>();

        board_numbers.iter().sum::<i32>() * drawn_numbers.last().unwrap()
    }
}

fn main() {
    let input = read_to_string("input/4").unwrap();

    let mut lines = input.lines().collect::<Vec<_>>();

    let numbers = lines
        .remove(0)
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let board_lines = lines.into_iter().collect::<Vec<_>>();

    let boards = board_lines
        .chunks(6)
        .map(|board_lines| {
            (1..6)
                .map(|y| {
                    board_lines[y]
                        .split_whitespace()
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(Board::new)
        .collect::<Vec<_>>();

    for number_index in 0..numbers.len() {
        let called_numbers = numbers[0..number_index].to_vec();
        let winning_board = boards.iter().find(|board| board.did_win(&called_numbers));

        if let Some(winning_board) = winning_board {
            let score = winning_board.score(&called_numbers);

            assert_eq!(16674, score);

            println!("{}", score);
            return;
        }
    }
    panic!("No winning board found");
}
