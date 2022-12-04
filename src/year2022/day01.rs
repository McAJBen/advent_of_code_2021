use crate::utils::read_input;

pub fn part1() -> u32 {
    read_input(2022, 1)
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

pub fn part2() -> u32 {
    let mut sections: Vec<u32> = read_input(2022, 1)
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
fn test_part1() {
    assert_eq!(part1(), 66186);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 196804);
}
