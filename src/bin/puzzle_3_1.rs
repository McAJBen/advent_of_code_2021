use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/3").unwrap();

    let lines: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|char| char == '1').collect())
        .collect();

    let num_bits = lines[0].len();
    let total_lines = lines.len();

    let mut gamma_rate = 0;

    for position in 0..num_bits {
        let num_ones = lines.iter().filter(|line| line[position]).count();

        gamma_rate <<= 1;
        if num_ones > total_lines / 2 {
            gamma_rate |= 1;
        }
    }

    let epsilon_rate = (i32::MAX >> (31 - num_bits)) ^ gamma_rate;

    let total = gamma_rate * epsilon_rate;

    assert_eq!(3959450, total);

    println!("{}", gamma_rate * epsilon_rate);
}
