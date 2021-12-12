use advent_of_code::ZipWithNextExt;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("puzzle_1_input").unwrap();

    let num_increases = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .zip_with_next()
        .filter(|(left, right)| left < right)
        .count();

    assert_eq!(1583, num_increases);

    println!("{}", num_increases);
}
