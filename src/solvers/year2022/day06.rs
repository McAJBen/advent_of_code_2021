use crate::{
    solvers::{Solver, SolverTrait},
    utils::ZipWithNextNExt,
};
use std::collections::HashSet;

impl SolverTrait for Solver<2022, 6, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        let x = input
            .chars()
            .zip_with_next_n(4)
            .enumerate()
            .filter_map(|(index, chunk)| {
                if chunk.iter().collect::<HashSet<_>>().len() == 4 {
                    return Some(index + 4);
                }
                None
            })
            .collect::<Vec<_>>();

        x[0]
    }
}

impl SolverTrait for Solver<2022, 6, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        let x = input
            .chars()
            .zip_with_next_n(14)
            .enumerate()
            .filter_map(|(index, chunk)| {
                if chunk.iter().collect::<HashSet<_>>().len() == 14 {
                    return Some(index + 14);
                }
                None
            })
            .collect::<Vec<_>>();

        x[0]
    }
}
