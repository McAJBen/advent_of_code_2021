use crate::utils::ZipWithNextNExt;
use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
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

pub fn part2(input: &str) -> usize {
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
