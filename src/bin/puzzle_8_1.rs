use std::{collections::HashSet, fs::read_to_string};

struct Display {
    outputs: Vec<HashSet<char>>,
}

impl Display {
    fn new(line: &str) -> Self {
        let (_, outputs) = line.split_once("|").unwrap();
        Self {
            outputs: outputs
                .split_whitespace()
                .map(|output| output.to_string().chars().collect())
                .collect(),
        }
    }
}

fn main() {
    let input = read_to_string("input/8").unwrap();

    let displays = input.lines().map(Display::new).collect::<Vec<_>>();

    // number of times that 1, 4, 7, 8 appear
    let count = displays
        .iter()
        .map(|display| {
            display
                .outputs
                .iter()
                .filter(|output| {
                    let num_segments = output.len();
                    num_segments == 2 || num_segments == 4 || num_segments == 3 || num_segments == 7
                })
                .count()
        })
        .sum::<usize>();

    assert_eq!(449, count);

    println!("{}", count);
}
