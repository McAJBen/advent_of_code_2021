use crate::solvers::{Solver, SolverTrait};

impl SolverTrait for Solver<2022, 1, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        input
            .split("\n\n")
            .map(|section| {
                section
                    .lines()
                    .map(|line| line.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .max()
            .unwrap()
    }
}

impl SolverTrait for Solver<2022, 1, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        let mut sections: Vec<u32> = input
            .split("\n\n")
            .map(|section| {
                section
                    .lines()
                    .map(|line| line.parse::<u32>().unwrap())
                    .sum()
            })
            .collect();

        sections.sort();
        sections.reverse();

        sections.into_iter().take(3).sum::<u32>()
    }
}
