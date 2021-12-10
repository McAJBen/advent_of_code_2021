use advent_of_code::{read_lines, ZipWithNextExt, ZipWithNextNExt};
use std::fs::File;

fn main() {
    let input = File::open("puzzle_1_input").unwrap();

    let num_increases = read_lines(&input)
        .into_iter()
        .map(|line| line.parse::<u32>().unwrap())
        .zip_with_next_n(3)
        .map(|group| group.iter().sum::<u32>())
        .collect::<Vec<_>>()
        .into_iter()
        .zip_with_next()
        .filter(|(left, right)| left < right)
        .count();

    println!("{}", num_increases);
}
