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
