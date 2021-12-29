use crate::utils::{read_input, ZipWithNextExt, ZipWithNextNExt};

pub fn part1() -> usize {
    read_input(2021, 1)
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .zip_with_next()
        .filter(|(left, right)| left < right)
        .count()
}

pub fn part2() -> usize {
    read_input(2021, 1)
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .zip_with_next_n(3)
        .map(|group| group.iter().sum::<u16>())
        .zip_with_next()
        .filter(|(left, right)| left < right)
        .count()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 1583);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 1627);
}
