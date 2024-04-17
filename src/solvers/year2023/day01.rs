use crate::solvers::{Solver, SolverTrait};

impl SolverTrait<u32> for Solver<2023, 1, 1> {
    fn solve(&self, input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
                let last_digit = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();

                first_digit * 10 + last_digit
            })
            .sum()
    }
}

impl SolverTrait<u32> for Solver<2023, 1, 2> {
    fn solve(&self, input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                let parse_number = |i: usize| {
                    if line[i..].starts_with("one") {
                        Some(1)
                    } else if line[i..].starts_with("two") {
                        Some(2)
                    } else if line[i..].starts_with("three") {
                        Some(3)
                    } else if line[i..].starts_with("four") {
                        Some(4)
                    } else if line[i..].starts_with("five") {
                        Some(5)
                    } else if line[i..].starts_with("six") {
                        Some(6)
                    } else if line[i..].starts_with("seven") {
                        Some(7)
                    } else if line[i..].starts_with("eight") {
                        Some(8)
                    } else if line[i..].starts_with("nine") {
                        Some(9)
                    } else {
                        None
                    }
                };

                let first_digit = line
                    .chars()
                    .enumerate()
                    .find_map(|(i, c)| c.to_digit(10).or_else(|| parse_number(i)))
                    .unwrap();
                let last_digit = line
                    .chars()
                    .rev()
                    .enumerate()
                    .find_map(|(i, c)| c.to_digit(10).or_else(|| parse_number(line.len() - i - 1)))
                    .unwrap();

                first_digit * 10 + last_digit
            })
            .sum()
    }
}
