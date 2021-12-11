use advent_of_code::read_lines;
use core::panic;
use std::fs::File;

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

enum ErrorType {
    Corruption(u64),
    Incomplete(u64),
}

fn parse_line(line: &str) -> ErrorType {
    let mut stack = Vec::new();
    for c in line.chars() {
        let (bracket, bracket_type) = Bracket::from_char(c);
        match bracket_type {
            BracketType::Open => stack.push(bracket),
            BracketType::Close => {
                let last = stack.pop().unwrap();
                if last != bracket {
                    return ErrorType::Corruption(bracket.corruption_value());
                }
            }
        }
    }

    if stack.len() > 0 {
        stack.reverse();
        ErrorType::Incomplete(
            stack
                .into_iter()
                .fold(0, |acc, bracket| 5 * acc + bracket.incomplete_value()),
        )
    } else {
        panic!("Empty stack")
    }
}

fn main() {
    let input = File::open("puzzle_10_input").unwrap();

    let lines = read_lines(&input);

    let mut incomplete_values = lines
        .into_iter()
        .filter_map(|line| match parse_line(&line) {
            ErrorType::Corruption(_) => None,
            ErrorType::Incomplete(i) => Some(i),
        })
        .collect::<Vec<_>>();

    println!("{:?}", incomplete_values);

    incomplete_values.sort();

    let middle = incomplete_values[incomplete_values.len() / 2];

    println!("{:?}", middle);
}
