use crate::utils::{ZipWithNextExt, ZipWithNextNExt};

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .zip_with_next()
        .filter(|(left, right)| left < right)
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .zip_with_next_n(3)
        .map(|group| group.iter().sum::<u16>())
        .zip_with_next()
        .filter(|(left, right)| left < right)
        .count()
}
