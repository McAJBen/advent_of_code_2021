use crate::utils::{read_input, ZipWithNextExt, ZipWithNextNExt};

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .zip_with_next()
        .filter(|(left, right)| left < right)
        .count()
}

#[test]
fn test_part1() {
    let input = read_input(2021, 1);
    assert_eq!(part1(&input), 1583);
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

#[test]
fn test_part2() {
    let input = read_input(2021, 1);
    assert_eq!(part2(&input), 1627);
}
