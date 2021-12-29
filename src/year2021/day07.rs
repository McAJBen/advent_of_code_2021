use crate::utils::read_input;

pub fn part1() -> u32 {
    let input = read_input(2021, 7);

    let positions: Vec<u16> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let max_position = *positions.iter().max().unwrap();
    let min_position = *positions.iter().min().unwrap();

    (min_position..=max_position)
        .map(|middle| {
            positions
                .iter()
                .map(|&p| if middle > p { middle - p } else { p - middle } as u32)
                .sum::<u32>()
        })
        .enumerate()
        .min_by_key(|&(_, x)| x)
        .unwrap()
        .1
}

pub fn part2() -> u32 {
    let input = read_input(2021, 7);

    let positions: Vec<u16> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let max_position = *positions.iter().max().unwrap();
    let min_position = *positions.iter().min().unwrap();

    (min_position..=max_position)
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
        .1
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 354129);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 98905973);
}
