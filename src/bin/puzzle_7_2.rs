use advent_of_code::read_lines;
use std::fs::File;

fn main() {
    let input = File::open("puzzle_7_input").unwrap();

    let positions: Vec<u16> = read_lines(&input)[0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let max_position = *positions.iter().max().unwrap();
    let min_position = *positions.iter().min().unwrap();

    println!("{} {}", min_position, max_position);

    let position_fuel = (min_position..=max_position)
        .map(|middle| {
            positions
                .iter()
                .map(|&p| {
                    let distance = if middle > p { middle - p } else { p - middle } as u128;

                    distance * (distance + 1) / 2
                })
                .sum::<u128>()
        })
        .enumerate()
        .collect::<Vec<_>>();

    println!("{:?}", position_fuel);

    let best = position_fuel.into_iter().min_by_key(|&(_, x)| x).unwrap();

    println!("{:?}", best);
}
