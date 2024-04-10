enum BracketType {
    Open,
    Close,
}

#[derive(Debug, PartialEq, Eq)]
enum Bracket {
    Parenthesis,
    Square,
    Curly,
    Angle,
}

impl Bracket {
    fn from_char(c: char) -> (Bracket, BracketType) {
        match c {
            '(' => (Bracket::Parenthesis, BracketType::Open),
            ')' => (Bracket::Parenthesis, BracketType::Close),
            '[' => (Bracket::Square, BracketType::Open),
            ']' => (Bracket::Square, BracketType::Close),
            '{' => (Bracket::Curly, BracketType::Open),
            '}' => (Bracket::Curly, BracketType::Close),
            '<' => (Bracket::Angle, BracketType::Open),
            '>' => (Bracket::Angle, BracketType::Close),
            _ => panic!("Invalid character"),
        }
    }

    fn corruption_value(&self) -> u64 {
        match self {
            Bracket::Parenthesis => 3,
            Bracket::Square => 57,
            Bracket::Curly => 1197,
            Bracket::Angle => 25137,
        }
    }

    fn incomplete_value(&self) -> u64 {
        match self {
            Bracket::Parenthesis => 1,
            Bracket::Square => 2,
            Bracket::Curly => 3,
            Bracket::Angle => 4,
        }
    }
}

fn find_corruption(line: &str) -> Option<Bracket> {
    let mut stack = Vec::new();
    for c in line.chars() {
        let (bracket, bracket_type) = Bracket::from_char(c);
        match bracket_type {
            BracketType::Open => stack.push(bracket),
            BracketType::Close => {
                let last = stack.pop().unwrap();
                if last != bracket {
                    return Some(bracket);
                }
            }
        }
    }
    None
}

fn parse_line(line: &str) -> Option<u64> {
    let mut stack = Vec::new();
    for c in line.chars() {
        let (bracket, bracket_type) = Bracket::from_char(c);
        match bracket_type {
            BracketType::Open => stack.push(bracket),
            BracketType::Close => {
                let last = stack.pop().unwrap();
                if last != bracket {
                    return None;
                }
            }
        }
    }

    if !stack.is_empty() {
        stack.reverse();
        Some(
            stack
                .into_iter()
                .fold(0, |acc, bracket| 5 * acc + bracket.incomplete_value()),
        )
    } else {
        panic!("Empty stack")
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(find_corruption)
        .map(|bracket| bracket.corruption_value())
        .sum::<u64>()
}

pub fn part2(input: &str) -> u64 {
    let mut incomplete_values = input
        .lines()
        .filter_map(|line| parse_line(line))
        .collect::<Vec<_>>();

    incomplete_values.sort_unstable();

    incomplete_values[incomplete_values.len() / 2]
}
