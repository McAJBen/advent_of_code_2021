use advent_of_code::{read_lines, Direction, DirectionCommand};
use std::fs::File;

fn main() {
    let input = File::open("puzzle_2_input").unwrap();

    let directions = read_lines(&input)
        .into_iter()
        .map(DirectionCommand::new)
        .collect::<Vec<_>>();

    let horizontal_position: i32 = directions
        .iter()
        .filter(|d| d.direction == Direction::Forward)
        .map(|d| d.amount)
        .sum();

    let depth: i32 = directions
        .into_iter()
        .filter_map(|d| match d.direction {
            Direction::Forward => None,
            Direction::Down => Some(d.amount),
            Direction::Up => Some(-d.amount),
        })
        .sum();

    let total = horizontal_position * depth;

    println!("{}", total);
}
