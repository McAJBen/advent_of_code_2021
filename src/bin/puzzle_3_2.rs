use advent_of_code::read_lines;
use std::fs::File;

fn main() {
    let input = File::open("puzzle_3_input").unwrap();

    let lines = read_lines(&input);

    let num_bits = lines[0].len();

    let mut oxygen_lines = lines.clone();

    for position in 0..num_bits {
        if oxygen_lines.len() == 1 {
            break;
        }

        let num_ones = oxygen_lines
            .iter()
            .filter(|line| line.chars().nth(position).unwrap() == '1')
            .count();

        let correct_value = if num_ones * 2 >= oxygen_lines.len() {
            // 1 is more common or equal
            '1'
        } else {
            '0'
        };

        oxygen_lines = oxygen_lines
            .into_iter()
            .filter(|line| line.chars().nth(position).unwrap() == correct_value)
            .collect();
    }

    let oxygen_rate = i32::from_str_radix(&oxygen_lines[0], 2).unwrap();

    let mut co2_lines = lines.clone();

    for position in 0..num_bits {
        if co2_lines.len() == 1 {
            break;
        }

        let num_ones = co2_lines
            .iter()
            .filter(|line| line.chars().nth(position).unwrap() == '1')
            .count();

        let correct_value = if num_ones * 2 >= co2_lines.len() {
            // 1 is more common or equal
            '0'
        } else {
            '1'
        };

        co2_lines = co2_lines
            .into_iter()
            .filter(|line| line.chars().nth(position).unwrap() == correct_value)
            .collect();
    }
    let co2_rate = i32::from_str_radix(&co2_lines[0], 2).unwrap();

    println!("{} {} {}", oxygen_rate, co2_rate, oxygen_rate * co2_rate);
}
