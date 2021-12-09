use advent_of_code::read_lines;
use std::{collections::HashSet, fs::File};

fn parse_line(line: &str) -> usize {
    let (patterns, outputs) = line.split_once("|").unwrap();
    let mut patterns = patterns
        .split_whitespace()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .collect::<Vec<_>>();

    patterns.sort_by_key(|pattern| pattern.len());

    let pattern_1 = patterns.remove(0);
    let pattern_7 = patterns.remove(0);
    let pattern_4 = patterns.remove(0);
    let pattern_8 = patterns.pop().unwrap();

    let (patterns_2_3_5, pattern_0_6_9) = patterns
        .into_iter()
        .partition::<Vec<_>, _>(|pattern| pattern.len() == 5);

    let (mut pattern_3, patterns_2_5) = patterns_2_3_5
        .into_iter()
        .partition::<Vec<_>, _>(|pattern| pattern.is_superset(&pattern_1));
    let pattern_3 = pattern_3.pop().unwrap();

    let (mut pattern_9, patterns_0_6) = pattern_0_6_9
        .into_iter()
        .partition::<Vec<_>, _>(|pattern| pattern.is_superset(&pattern_3));
    let pattern_9 = pattern_9.pop().unwrap();

    let (mut pattern_0, mut pattern_6) = patterns_0_6
        .into_iter()
        .partition::<Vec<_>, _>(|pattern| pattern.is_superset(&pattern_1));
    let pattern_0 = pattern_0.pop().unwrap();
    let pattern_6 = pattern_6.pop().unwrap();

    let (mut pattern_5, mut pattern_2) = patterns_2_5
        .into_iter()
        .partition::<Vec<_>, _>(|pattern| pattern.is_subset(&pattern_6));
    let pattern_5 = pattern_5.pop().unwrap();
    let pattern_2 = pattern_2.pop().unwrap();

    let patterns = [
        pattern_0, pattern_1, pattern_2, pattern_3, pattern_4, pattern_5, pattern_6, pattern_7,
        pattern_8, pattern_9,
    ];

    outputs
        .split_whitespace()
        .map(|s| {
            let output = s.chars().collect::<HashSet<char>>();

            patterns
                .iter()
                .enumerate()
                .find(|(_, pattern)| pattern.is_subset(&output) && output.is_subset(pattern))
                .unwrap()
                .0
        })
        .fold(0, |prev, digit| digit + prev * 10)
}

fn main() {
    let input = File::open("puzzle_8_input").unwrap();

    let x: usize = read_lines(&input)
        .into_iter()
        .map(|line| parse_line(&line))
        .sum();

    println!("{}", x);
}
