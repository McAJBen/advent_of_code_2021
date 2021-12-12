use std::fs::read_to_string;

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

    fn value(&self) -> u32 {
        match self {
            Bracket::Parenthesis => 3,
            Bracket::Square => 57,
            Bracket::Curly => 1197,
            Bracket::Angle => 25137,
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

fn main() {
    let input = read_to_string("puzzle_10_input").unwrap();

    let total = input
        .lines()
        .filter_map(find_corruption)
        .map(|bracket| bracket.value())
        .sum::<u32>();

    assert_eq!(392097, total);

    println!("{}", total);
}
