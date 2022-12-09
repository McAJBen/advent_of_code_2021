use crate::utils::read_input;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap() / 3 - 2)
        .sum::<u32>()
}

#[test]
fn test_part1() {
    let input = read_input(2019, 1);
    assert_eq!(part1(&input), 3403509);
}

pub fn part2(input: &str) -> u32 {
    let mut masses = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut total_fuel = 0;

    while let Some(mass) = masses.pop() {
        let fuel = (mass / 3).saturating_sub(2);
        total_fuel += fuel;
        if fuel > 0 {
            masses.push(fuel);
        }
    }

    total_fuel
}

#[test]
fn test_part2() {
    let input = read_input(2019, 1);
    assert_eq!(part2(&input), 5102369);
}
