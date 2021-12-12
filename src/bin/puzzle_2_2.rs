use advent_of_code::{Direction, DirectionCommand};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("puzzle_2_input").unwrap();

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for direction in input.lines().map(DirectionCommand::new) {
        match direction.direction {
            Direction::Forward => {
                horizontal_position += direction.amount;
                depth += aim * direction.amount;
            }
            Direction::Down => aim += direction.amount,
            Direction::Up => aim -= direction.amount,
        }
    }

    let total = horizontal_position * depth;

    assert_eq!(2006917119, total);

    println!("{}", total);
}
