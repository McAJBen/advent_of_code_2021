use std::fs::read_to_string;

fn main() {
    let input = read_to_string("puzzle_7_input").unwrap();

    let positions: Vec<u16> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let max_position = *positions.iter().max().unwrap();
    let min_position = *positions.iter().min().unwrap();

    let position_fuel = (min_position..=max_position)
        .map(|middle| {
            positions
                .iter()
                .map(|&p| if middle > p { middle - p } else { p - middle } as u128)
                .sum::<u128>()
        })
        .collect::<Vec<_>>();

    let total = position_fuel
        .into_iter()
        .enumerate()
        .min_by_key(|&(_, x)| x)
        .unwrap()
        .1;

    assert_eq!(354129, total);

    println!("{}", total);
}
