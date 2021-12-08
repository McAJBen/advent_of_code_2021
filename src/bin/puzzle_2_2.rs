use advent_of_code::{read_lines, Direction, DirectionCommand};
use std::fs::File;

fn main() {
    let input = File::open("puzzle_2_input").unwrap();

    let directions = read_lines(&input)
        .into_iter()
        .map(|line| DirectionCommand::new(line))
        .collect::<Vec<_>>();

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for direction in directions {
        match direction.direction {
            Direction::Forward => {
                horizontal_position += direction.amount;
                depth += aim * direction.amount;
            }
            Direction::Down => aim += direction.amount,
            Direction::Up => aim -= direction.amount,
        }
    }

    println!(
        "{} {} {}",
        horizontal_position,
        depth,
        horizontal_position * depth
    );
}
