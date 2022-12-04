use crate::utils::read_input;

pub fn part1() -> u32 {
    read_input(2022, 2)
        .lines()
        .map(|line| match line {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => panic!(),
        })
        .sum()
}

pub fn part2() -> u32 {
    read_input(2022, 2)
        .lines()
        .map(|line| match line {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => panic!(),
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 12740);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 11980);
}
