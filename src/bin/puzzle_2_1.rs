use advent_of_code::{Direction, DirectionCommand};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("puzzle_2_input").unwrap();

    let directions = input.lines().map(DirectionCommand::new).collect::<Vec<_>>();

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

    assert_eq!(1989014, total);

    println!("{}", total);
}
