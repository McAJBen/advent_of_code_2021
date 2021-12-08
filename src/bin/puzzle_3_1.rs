use advent_of_code::read_lines;
use std::fs::File;

fn main() {
    let input = File::open("puzzle_3_input").unwrap();

    let lines = read_lines(&input);

    let num_bits = lines[0].len();
    let total_lines = lines.len();

    let gamma_rate_str = (0..num_bits)
        .map(|position| {
            let num_ones = lines
                .iter()
                .filter(|line| line.chars().nth(position).unwrap() == '1')
                .count();

            if num_ones * 2 > total_lines {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();

    let gamma_rate = i32::from_str_radix(&gamma_rate_str, 2).unwrap();

    let epsilon_rate_str = gamma_rate_str
        .chars()
        .map(|c| match c {
            '1' => '0',
            '0' => '1',
            _ => panic!("Invalid character"),
        })
        .collect::<String>();

    let epsilon_rate = i32::from_str_radix(&epsilon_rate_str, 2).unwrap();

    println!(
        "{} {} {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );
}
