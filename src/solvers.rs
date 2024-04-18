mod year2019 {
    mod day01;
    mod day02;
}
mod year2021 {
    mod day01;
    mod day02;
    mod day03;
    mod day04;
    mod day05;
    mod day06;
    mod day07;
    mod day08;
    mod day09;
    mod day10;
    mod day11;
    mod day12;
    mod day13;
    mod day14;
    mod day15;
    mod day16;
    mod day17;
    mod day18;
    mod day19;
    mod day20;
    mod day21;
    mod day22;
    mod day23;
    mod day24;
    mod day25;
}
mod year2022 {
    mod day01;
    mod day02;
    mod day03;
    mod day04;
    mod day05;
    mod day06;
    mod day07;
    mod day08;
    mod day09;
    mod day10;
    mod day11;
    mod day12;
    mod day13;
    mod day14;
    mod day15;
    mod day16;
    mod day17;
    mod day18;
    mod day19;
    mod day20;
    mod day21;
    mod day22;
    mod day23;
    mod day24;
    mod day25;
}
mod year2023 {
    mod day01;
}

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
