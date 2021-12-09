use advent_of_code::read_lines;
use std::{collections::HashSet, fs::File};

struct Display {
    patterns: Vec<HashSet<char>>,
    outputs: Vec<HashSet<char>>,
}

impl Display {
    fn new(line: String) -> Self {
        let (patterns, outputs) = line.split_once("|").unwrap();
        Self {
            patterns: patterns
                .split_whitespace()
                .map(|pattern| pattern.to_string().chars().collect())
                .collect(),
            outputs: outputs
                .split_whitespace()
                .map(|output| output.to_string().chars().collect())
                .collect(),
        }
    }
}

fn main() {
    let input = File::open("puzzle_8_input").unwrap();

    let displays = read_lines(&input)
        .into_iter()
        .map(Display::new)
        .collect::<Vec<_>>();

    // number of times that 1, 4, 7, 8 appear
    let count: u128 = displays
        .iter()
        .map(|display| {
            display
                .outputs
                .iter()
                .filter(|output| {
                    let num_segments = output.len();
                    num_segments == 2 || num_segments == 4 || num_segments == 3 || num_segments == 7
                })
                .count() as u128
        })
        .sum();

    println!("{}", count);
}
