use crate::utils::{read_input, ZipWithNextNExt};
use std::collections::HashSet;

pub fn part1() -> usize {
    let input = read_input(2022, 6);
    let x = input
        .chars()
        .zip_with_next_n(4)
        .enumerate()
        .filter_map(|(index, chunk)| {
            if chunk.iter().collect::<HashSet<_>>().len() == 4 {
                return Some(index + 4);
            }
            return None;
        })
        .collect::<Vec<_>>();

    x[0]
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 1235);
}

pub fn part2() -> usize {
    let input = read_input(2022, 6);
    let x = input
        .chars()
        .zip_with_next_n(14)
        .enumerate()
        .filter_map(|(index, chunk)| {
            if chunk.iter().collect::<HashSet<_>>().len() == 14 {
                return Some(index + 14);
            }
            return None;
        })
        .collect::<Vec<_>>();

    x[0]
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 3051);
}
