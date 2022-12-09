use crate::utils::read_input;

pub fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

#[test]
fn test_part1() {
    let input = read_input(2022, 1);
    assert_eq!(part1(&input), 66186);
}

pub fn part2(input: &str) -> u32 {
    let mut sections: Vec<u32> = input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    sections.sort();
    sections.reverse();

    sections.into_iter().take(3).sum()
}

#[test]
fn test_part2() {
    let input = read_input(2022, 1);
    assert_eq!(part2(&input), 196804);
}
