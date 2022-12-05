use crate::utils::read_input;

pub fn part1() -> String {
    let input = read_input(2022, 5);
    let (original, directions) = input.split_once("\n\n").unwrap();

    let mut positions = Vec::new();
    for line in original.lines() {
        let chars: Vec<char> = line.chars().collect();
        for (row, chunk) in chars.chunks(4).enumerate() {
            if chunk[0] == '[' {
                positions.push((row, chunk[1]));
            }
        }
    }

    positions.reverse();

    let num_stacks = positions.iter().map(|(c, _)| c).max().unwrap();

    let mut stacks = vec![Vec::<char>::new(); *num_stacks + 1];

    for position in positions {
        stacks[position.0].push(position.1);
    }

    for direction in directions.lines() {
        let direction = direction.strip_prefix("move ").unwrap();
        let (num, end) = direction.split_once(" from ").unwrap();
        let (from, to) = end.split_once(" to ").unwrap();

        let num = num.parse().unwrap();
        let from = from.parse::<usize>().unwrap() - 1;
        let to = to.parse::<usize>().unwrap() - 1;

        for _ in 0..num {
            let value = stacks[from].pop().unwrap();
            stacks[to].push(value);
        }
    }

    stacks
        .into_iter()
        .map(|stack| stack.last().copied().unwrap())
        .collect::<String>()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), "CFFHVVHNC");
}

pub fn part2() -> String {
    let input = read_input(2022, 5);
    let (original, directions) = input.split_once("\n\n").unwrap();

    let mut positions = Vec::new();
    for line in original.lines() {
        let chars: Vec<char> = line.chars().collect();
        for (row, chunk) in chars.chunks(4).enumerate() {
            if chunk[0] == '[' {
                positions.push((row, chunk[1]));
            }
        }
    }

    positions.reverse();

    let num_stacks = positions.iter().map(|(c, _)| c).max().unwrap();

    let mut stacks = vec![Vec::<char>::new(); *num_stacks + 1];

    for position in positions {
        stacks[position.0].push(position.1);
    }

    for direction in directions.lines() {
        let direction = direction.strip_prefix("move ").unwrap();
        let (num, end) = direction.split_once(" from ").unwrap();
        let (from, to) = end.split_once(" to ").unwrap();

        let num = num.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap() - 1;
        let to = to.parse::<usize>().unwrap() - 1;

        let len = stacks[from].len() - num;

        let values = stacks[from].split_off(len);

        stacks[to].extend(values);
    }

    stacks
        .into_iter()
        .map(|stack| stack.last().copied().unwrap())
        .collect::<String>()
}

#[test]
fn test_part2() {
    assert_eq!(part2(), "FSZWBPTBG");
}
