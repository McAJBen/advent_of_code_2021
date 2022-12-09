use crate::utils::read_input;
use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let items: Vec<u8> = line
                .chars()
                .map(|c| match c {
                    'a'..='z' => c as u8 - 0x60,
                    'A'..='Z' => c as u8 - 0x40 + 26,
                    _ => panic!(),
                })
                .collect();
            let (c1, c2) = items.split_at(items.len() / 2);

            let c1: HashSet<u8> = c1.iter().copied().collect();
            let c2: HashSet<u8> = c2.iter().copied().collect();

            let intersection: HashSet<u8> = c1.intersection(&c2).copied().collect();
            intersection.into_iter().map(|u8| u8 as u32).sum::<u32>()
        })
        .sum()
}

#[test]
fn test_part1() {
    let input = read_input(2022, 3);
    assert_eq!(part1(&input), 8176);
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|lines| {
            lines
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            'a'..='z' => c as u8 - 0x60,
                            'A'..='Z' => c as u8 - 0x40 + 26,
                            _ => panic!(),
                        })
                        .collect::<HashSet<u8>>()
                })
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .unwrap()
                .into_iter()
                .map(|u8| u8 as u32)
                .sum::<u32>()
        })
        .sum()
}

#[test]
fn test_part2() {
    let input = read_input(2022, 3);
    assert_eq!(part2(&input), 2689);
}
