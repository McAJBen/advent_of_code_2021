use advent_of_code_2021::{Direction, DirectionCommand};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/2").unwrap();

    let mut horizontal_position = 0;
    let mut depth = 0;

    for direction in input.lines().map(DirectionCommand::new) {
        match direction.direction {
            Direction::Forward => {
                horizontal_position += direction.amount;
            }
            Direction::Down => depth += direction.amount,
            Direction::Up => depth -= direction.amount,
        }
    }

    let total = horizontal_position * depth;

    assert_eq!(1989014, total);

    println!("{}", total);
}
