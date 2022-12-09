use crate::utils::read_input;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| match line {
            "A X" => 4, // 3 + 1
            "A Y" => 8, // 6 + 2
            "A Z" => 3, // 0 + 3
            "B X" => 1, // 0 + 1
            "B Y" => 5, // 3 + 2
            "B Z" => 9, // 6 + 3
            "C X" => 7, // 6 + 1
            "C Y" => 2, // 0 + 2
            "C Z" => 6, // 3 + 3
            _ => panic!(),
        })
        .sum()
}

#[test]
fn test_part1() {
    let input = read_input(2022, 2);
    assert_eq!(part1(&input), 12740);
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| match line {
            "A X" => 3, // 0 + 3
            "A Y" => 4, // 3 + 1
            "A Z" => 8, // 6 + 2
            "B X" => 1, // 0 + 1
            "B Y" => 5, // 3 + 2
            "B Z" => 9, // 6 + 3
            "C X" => 2, // 0 + 2
            "C Y" => 6, // 3 + 3
            "C Z" => 7, // 6 + 1
            _ => panic!(),
        })
        .sum()
}

#[test]
fn test_part2() {
    let input = read_input(2022, 2);
    assert_eq!(part2(&input), 11980);
}
