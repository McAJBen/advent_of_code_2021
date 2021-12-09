use advent_of_code::{read_lines, ZipWithNextExt};
use std::fs::File;

fn main() {
    let input = File::open("puzzle_1_input").unwrap();

    let num_increases = read_lines(&input)
        .into_iter()
        .map(|line| line.parse::<u32>().unwrap())
        .zip_with_next()
        .filter(|(left, right)| left < right)
        .count();

    println!("{}", num_increases);
}
