use crate::solvers::{Solver, SolverTrait};
use std::ops::RangeInclusive;

impl SolverTrait for Solver<2022, 4, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        input
            .lines()
            .filter(|line| {
                let (left, right) = line.split_once(',').unwrap();

                let left_range = left
                    .split_once('-')
                    .map(|(a, b)| {
                        RangeInclusive::new(a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
                    })
                    .unwrap();
                let right_range = right
                    .split_once('-')
                    .map(|(a, b)| {
                        RangeInclusive::new(a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
                    })
                    .unwrap();

                match left_range.start().cmp(right_range.start()) {
                    std::cmp::Ordering::Less => match left_range.end().cmp(right_range.end()) {
                        std::cmp::Ordering::Less => false,
                        std::cmp::Ordering::Equal => true,
                        std::cmp::Ordering::Greater => true,
                    },
                    std::cmp::Ordering::Equal => true,
                    std::cmp::Ordering::Greater => match left_range.end().cmp(right_range.end()) {
                        std::cmp::Ordering::Less => true,
                        std::cmp::Ordering::Equal => true,
                        std::cmp::Ordering::Greater => false,
                    },
                }
            })
            .count()
    }
}

impl SolverTrait for Solver<2022, 4, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        input
            .lines()
            .filter(|line| {
                let (left, right) = line.split_once(',').unwrap();

                let left_range = left
                    .split_once('-')
                    .map(|(a, b)| {
                        RangeInclusive::new(a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
                    })
                    .unwrap();
                let right_range = right
                    .split_once('-')
                    .map(|(a, b)| {
                        RangeInclusive::new(a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
                    })
                    .unwrap();

                left_range.start() <= right_range.end() && left_range.end() >= right_range.start()
            })
            .count()
    }
}
