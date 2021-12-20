use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/3").unwrap();

    let lines: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|char| char == '1').collect())
        .collect();

    let num_bits = lines[0].len();

    let mut oxygen_rate = 0;
    let mut oxygen_lines = lines.clone();

    for position in 0..num_bits {
        let bit = if oxygen_lines.len() != 1 {
            let num_ones = oxygen_lines.iter().filter(|line| line[position]).count();

            // 1 is more common or equal
            let correct_value = num_ones >= oxygen_lines.len() / 2;

            oxygen_lines = oxygen_lines
                .into_iter()
                .filter(|line| line[position] == correct_value)
                .collect();

            correct_value
        } else {
            oxygen_lines[0][position]
        };

        oxygen_rate <<= 1;
        if bit {
            oxygen_rate |= 1;
        }
    }

    let mut co2_rate = 0;
    let mut co2_lines = lines;

    for position in 0..num_bits {
        let bit = if co2_lines.len() != 1 {
            let num_ones = co2_lines.iter().filter(|line| line[position]).count();

            // 0 is less common
            let correct_value = num_ones < co2_lines.len() / 2;

            co2_lines = co2_lines
                .into_iter()
                .filter(|line| line[position] == correct_value)
                .collect();

            correct_value
        } else {
            co2_lines[0][position]
        };

        co2_rate <<= 1;
        if bit {
            co2_rate |= 1;
        }
    }

    let total = oxygen_rate * co2_rate;

    assert_eq!(7440311, total);

    println!("{}", total);
}
