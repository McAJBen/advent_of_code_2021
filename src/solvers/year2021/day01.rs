use crate::{
    solvers::{Solver, SolverTrait},
    utils::{ZipWithNextExt, ZipWithNextNExt},
};

impl SolverTrait for Solver<2021, 1, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        input
            .lines()
            .map(|line| line.parse::<u16>().unwrap())
            .zip_with_next()
            .filter(|(left, right)| left < right)
            .count()
    }
}

impl SolverTrait for Solver<2021, 1, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        input
            .lines()
            .map(|line| line.parse::<u16>().unwrap())
            .zip_with_next_n(3)
            .map(|group| group.iter().sum::<u16>())
            .zip_with_next()
            .filter(|(left, right)| left < right)
            .count()
    }
}
