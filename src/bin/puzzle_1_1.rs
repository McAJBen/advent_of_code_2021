use advent_of_code_2021::ZipWithNextExt;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/1").unwrap();

    let num_increases = input
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .zip_with_next()
        .filter(|(left, right)| left < right)
        .count();

    assert_eq!(1583, num_increases);

    println!("{}", num_increases);
}
