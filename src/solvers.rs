mod year2019;
mod year2021;
mod year2022;
mod year2023;

pub struct Solver<const YEAR: u16, const DAY: u8, const PART: u8>;

impl<const YEAR: u16, const DAY: u8, const PART: u8> Solver<YEAR, DAY, PART> {
    pub fn year(&self) -> u16 {
        YEAR
    }

    pub fn day(&self) -> u8 {
        DAY
    }

    pub fn part(&self) -> u8 {
        PART
    }
}

pub trait SolverTrait<OutputType> {
    fn solve(&self, input: &str) -> OutputType;
}
