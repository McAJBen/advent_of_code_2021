use crate::solvers::{Solver, SolverTrait};
use nom::{bytes::complete::take_while, character::complete::digit1, combinator::map_res, IResult};
use std::{collections::HashMap, iter::zip};

fn parse_input(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, left) = map_res(digit1, str::parse::<u64>)(input)?;
    let (input, _) = take_while(char::is_whitespace)(input)?;
    let (input, right) = map_res(digit1, str::parse::<u64>)(input)?;
    IResult::Ok((input, (left, right)))
}

impl SolverTrait for Solver<2024, 1, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        let (mut left, mut right): (Vec<u64>, Vec<u64>) = input
            .lines()
            .map(|line| parse_input(line).unwrap().1)
            .unzip();

        left.sort();
        right.sort();

        zip(left, right).map(|(a, b)| a.abs_diff(b)).sum::<u64>()
    }
}

impl SolverTrait for Solver<2024, 1, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        let (left, right): (Vec<u64>, Vec<u64>) = input
            .lines()
            .map(|line| parse_input(line).unwrap().1)
            .unzip();

        let left = left.into_iter().fold(HashMap::new(), |mut acc, value| {
            (*acc.entry(value).or_insert(0u64)) += 1;
            acc
        });

        let right = right.into_iter().fold(HashMap::new(), |mut acc, value| {
            (*acc.entry(value).or_insert(0u64)) += 1;
            acc
        });

        left.into_iter()
            .map(|(key, value)| key * value * right.get(&key).unwrap_or(&0u64))
            .sum::<u64>()
    }
}
