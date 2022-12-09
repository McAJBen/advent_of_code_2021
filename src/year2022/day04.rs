use crate::utils::read_input;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> usize {
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

#[test]
fn test_part1() {
    let input = read_input(2022, 4);
    assert_eq!(part1(&input), 644);
}

pub fn part2(input: &str) -> usize {
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

#[test]
fn test_part2() {
    let input = read_input(2022, 4);
    assert_eq!(part2(&input), 926);
}
