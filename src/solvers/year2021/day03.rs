use crate::solvers::{Solver, SolverTrait};

impl SolverTrait<i32> for Solver<2021, 3, 1> {
    fn solve(&self, input: &str) -> i32 {
        let lines: Vec<Vec<bool>> = input
            .lines()
            .map(|line| line.chars().map(|char| char == '1').collect())
            .collect();

        let num_bits = lines[0].len();
        let total_lines = lines.len();

        let mut gamma_rate = 0;

        for position in 0..num_bits {
            let num_ones = lines.iter().filter(|line| line[position]).count();

            gamma_rate <<= 1;
            if num_ones > total_lines / 2 {
                gamma_rate |= 1;
            }
        }

        let epsilon_rate = (i32::MAX >> (31 - num_bits)) ^ gamma_rate;

        gamma_rate * epsilon_rate
    }
}

impl SolverTrait<i32> for Solver<2021, 3, 2> {
    fn solve(&self, input: &str) -> i32 {
        let lines: Vec<Vec<bool>> = input
            .lines()
            .map(|line| line.chars().map(|char| char == '1').collect())
            .collect();

        let num_bits = lines[0].len();

        let mut oxygen_rate = 0;
        let mut oxygen_lines = lines.clone();

        for position in 0..num_bits {
            let bit = if oxygen_lines.len() != 1 {
                let num_ones = oxygen_lines.iter().filter(|line| line[position]).count();

                // 1 is more common or equal
                let correct_value = num_ones >= oxygen_lines.len() / 2;

                oxygen_lines.retain(|line| line[position] == correct_value);

                correct_value
            } else {
                oxygen_lines[0][position]
            };

            oxygen_rate <<= 1;
            if bit {
                oxygen_rate |= 1;
            }
        }

        let mut co2_rate = 0;
        let mut co2_lines = lines;

        for position in 0..num_bits {
            let bit = if co2_lines.len() != 1 {
                let num_ones = co2_lines.iter().filter(|line| line[position]).count();

                // 0 is less common
                let correct_value = num_ones < co2_lines.len() / 2;

                co2_lines.retain(|line| line[position] == correct_value);

                correct_value
            } else {
                co2_lines[0][position]
            };

            co2_rate <<= 1;
            if bit {
                co2_rate |= 1;
            }
        }

        oxygen_rate * co2_rate
    }
}
