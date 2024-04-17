use crate::{
    solvers::{Solver, SolverTrait},
    utils::ZipWithNextExt,
};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl SolverTrait<usize> for Solver<2022, 9, 1> {
    fn solve(&self, input: &str) -> usize {
        let orders: Vec<Direction> = input
            .lines()
            .flat_map(|line| {
                let (direction, count) = line.split_once(' ').unwrap();
                let direction = match direction {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!(),
                };
                let count = count.parse::<usize>().unwrap();

                vec![direction; count]
            })
            .collect();

        let mut head = (0, 0);
        let mut tail = (0, 0);

        let mut tail_history = HashSet::new();
        tail_history.insert(tail);

        for direction in orders {
            match direction {
                Direction::Up => head.1 -= 1,
                Direction::Down => head.1 += 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
            }

            match (head.0 - tail.0, head.1 - tail.1) {
                (-2, 0) => tail.0 -= 1,
                (2, 0) => tail.0 += 1,
                (0, -2) => tail.1 -= 1,
                (0, 2) => tail.1 += 1,

                (-2, -1) | (-1, -2) => {
                    tail.0 -= 1;
                    tail.1 -= 1
                }
                (-2, 1) | (-1, 2) => {
                    tail.0 -= 1;
                    tail.1 += 1;
                }
                (1, -2) | (2, -1) => {
                    tail.0 += 1;
                    tail.1 -= 1;
                }
                (2, 1) | (1, 2) => {
                    tail.0 += 1;
                    tail.1 += 1;
                }
                _ => {}
            }

            tail_history.insert(tail);
        }

        tail_history.len()
    }
}

impl SolverTrait<usize> for Solver<2022, 9, 2> {
    fn solve(&self, input: &str) -> usize {
        let orders: Vec<Direction> = input
            .lines()
            .flat_map(|line| {
                let (direction, count) = line.split_once(' ').unwrap();
                let direction = match direction {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!(),
                };
                let count = count.parse::<usize>().unwrap();

                vec![direction; count]
            })
            .collect();

        let mut snake = [(0, 0); 10];

        let mut tail_history = HashSet::new();
        tail_history.insert(*snake.last().unwrap());

        for direction in orders {
            {
                let head = snake.first_mut().unwrap();
                match direction {
                    Direction::Up => head.1 -= 1,
                    Direction::Down => head.1 += 1,
                    Direction::Left => head.0 -= 1,
                    Direction::Right => head.0 += 1,
                }
            }

            for (head, tail) in (0..snake.len()).zip_with_next() {
                let head = snake[head];
                let tail = &mut snake[tail];
                match (head.0 - tail.0, head.1 - tail.1) {
                    (-2, 0) => tail.0 -= 1,
                    (2, 0) => tail.0 += 1,
                    (0, -2) => tail.1 -= 1,
                    (0, 2) => tail.1 += 1,

                    (-2, -1) | (-1, -2) | (-2, -2) => {
                        tail.0 -= 1;
                        tail.1 -= 1
                    }
                    (-2, 1) | (-1, 2) | (-2, 2) => {
                        tail.0 -= 1;
                        tail.1 += 1;
                    }
                    (1, -2) | (2, -1) | (2, -2) => {
                        tail.0 += 1;
                        tail.1 -= 1;
                    }
                    (2, 1) | (1, 2) | (2, 2) => {
                        tail.0 += 1;
                        tail.1 += 1;
                    }
                    _ => {}
                }
            }

            tail_history.insert(*snake.last().unwrap());
        }

        tail_history.len()
    }
}
