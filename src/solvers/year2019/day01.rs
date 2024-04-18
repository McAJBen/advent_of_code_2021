use crate::solvers::{Solver, SolverTrait};

impl SolverTrait for Solver<2019, 1, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        input
            .lines()
            .map(|line| line.parse::<u32>().unwrap() / 3 - 2)
            .sum::<u32>()
    }
}

impl SolverTrait for Solver<2019, 1, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        let mut masses = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut total_fuel = 0;

        while let Some(mass) = masses.pop() {
            let fuel = (mass / 3).saturating_sub(2);
            total_fuel += fuel;
            if fuel > 0 {
                masses.push(fuel);
            }
        }

        total_fuel
    }
}
