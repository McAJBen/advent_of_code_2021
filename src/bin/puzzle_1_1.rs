use advent_of_code::{read_lines, ZipWithNext};
use std::fs::File;

fn main() {
    let input = File::open("puzzle_1_input").unwrap();

    let num_increases = ZipWithNext::new(
        read_lines(&input)
            .into_iter()
            .map(|line| line.parse::<u32>().unwrap()),
    )
    .filter(|(left, right)| left < right)
    .count();

    println!("{}", num_increases);
}
