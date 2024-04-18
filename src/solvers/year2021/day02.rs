use crate::solvers::{Solver, SolverTrait};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl Direction {
    fn from_str(string: &str) -> Direction {
        match string {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Unknown direction: {}", string),
        }
    }
}

struct DirectionCommand {
    direction: Direction,
    amount: i32,
}

impl DirectionCommand {
    pub fn new(cmd: &str) -> Self {
        let (left, right) = cmd.split_once(' ').unwrap();
        Self {
            direction: Direction::from_str(left),
            amount: right.parse::<i32>().unwrap(),
        }
    }
}

impl SolverTrait for Solver<2021, 2, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        let mut horizontal_position = 0;
        let mut depth = 0;

        for direction in input.lines().map(DirectionCommand::new) {
            match direction.direction {
                Direction::Forward => {
                    horizontal_position += direction.amount;
                }
                Direction::Down => depth += direction.amount,
                Direction::Up => depth -= direction.amount,
            }
        }

        horizontal_position * depth
    }
}

impl SolverTrait for Solver<2021, 2, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        let mut horizontal_position = 0;
        let mut depth = 0;
        let mut aim = 0;

        for direction in input.lines().map(DirectionCommand::new) {
            match direction.direction {
                Direction::Forward => {
                    horizontal_position += direction.amount;
                    depth += aim * direction.amount;
                }
                Direction::Down => aim += direction.amount,
                Direction::Up => aim -= direction.amount,
            }
        }

        horizontal_position * depth
    }
}
