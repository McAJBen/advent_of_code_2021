use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/7").unwrap();

    let positions: Vec<u16> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let max_position = *positions.iter().max().unwrap();
    let min_position = *positions.iter().min().unwrap();

    let total = (min_position..=max_position)
        .map(|middle| {
            positions
                .iter()
                .map(|&p| {
                    let distance = if middle > p { middle - p } else { p - middle } as u32;

                    distance * (distance + 1) / 2
                })
                .sum::<u32>()
        })
        .enumerate()
        .min_by_key(|&(_, x)| x)
        .unwrap()
        .1;

    assert_eq!(98905973, total);

    println!("{:?}", total);
}
