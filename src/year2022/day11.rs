use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    num_inspections: u64,
}

#[derive(Debug)]
enum Operation {
    Add(Argument, Argument),
    Multiply(Argument, Argument),
}

impl Operation {
    fn from_lines(lines: &[&str]) -> Option<Self> {
        let line = lines.iter().find(|l| l.contains("Operation:"))?;

        let statement = line.trim().strip_prefix("Operation: new = ")?;
        let segments: Vec<&str> = statement.split_whitespace().collect();

        let lhs = match segments[0] {
            "old" => Argument::Current,
            value => Argument::Scalar(value.parse().ok()?),
        };

        let rhs = match segments[2] {
            "old" => Argument::Current,
            value => Argument::Scalar(value.parse().ok()?),
        };

        match segments[1] {
            "+" => Some(Operation::Add(lhs, rhs)),
            "*" => Some(Operation::Multiply(lhs, rhs)),
            _ => None,
        }
    }

    fn run(&self, input: u64) -> u64 {
        match self {
            Operation::Add(lhs, rhs) => lhs.unwrap_or(input) + rhs.unwrap_or(input),
            Operation::Multiply(lhs, rhs) => lhs.unwrap_or(input) * rhs.unwrap_or(input),
        }
    }
}

#[derive(Debug)]
enum Argument {
    Current,
    Scalar(u64),
}

impl Argument {
    fn unwrap_or(&self, value: u64) -> u64 {
        match self {
            Argument::Current => value,
            Argument::Scalar(value) => *value,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisor: u64,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn from_lines(lines: &[&str]) -> Option<Self> {
        let divisor = lines
            .iter()
            .find(|l| l.contains("Test:"))?
            .trim()
            .strip_prefix("Test: divisible by ")?
            .parse()
            .ok()?;
        let if_true = lines
            .iter()
            .find(|l| l.contains("If true:"))?
            .trim()
            .strip_prefix("If true: throw to monkey ")?
            .parse()
            .ok()?;
        let if_false = lines
            .iter()
            .find(|l| l.contains("If false:"))?
            .trim()
            .strip_prefix("If false: throw to monkey ")?
            .parse()
            .ok()?;
        Some(Self {
            divisor,
            if_true,
            if_false,
        })
    }

    fn run(&self, input: u64) -> usize {
        if input % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut monkies: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey| {
            let lines: Vec<&str> = monkey.lines().collect();

            Monkey {
                items: lines
                    .iter()
                    .find(|l| l.contains("Starting items:"))
                    .unwrap()
                    .trim()
                    .strip_prefix("Starting items: ")
                    .unwrap()
                    .split(',')
                    .map(|num| num.trim().parse().unwrap())
                    .collect(),
                operation: Operation::from_lines(&lines).unwrap(),
                test: Test::from_lines(&lines).unwrap(),
                num_inspections: 0,
            }
        })
        .collect();

    for _ in 0..20 {
        let mut monkey_index = 0;
        while monkey_index < monkies.len() {
            while let Some(item) = monkies[monkey_index].items.pop_front() {
                monkies[monkey_index].num_inspections += 1;
                let new_item = monkies[monkey_index].operation.run(item) / 3;
                let new_monkey_index = monkies[monkey_index].test.run(new_item);
                monkies[new_monkey_index].items.push_back(new_item);
            }
            monkey_index += 1;
        }
    }

    let mut num_inspections: Vec<u64> = monkies.into_iter().map(|m| m.num_inspections).collect();
    num_inspections.sort();

    num_inspections.pop().unwrap() * num_inspections.pop().unwrap()
}

pub fn part2(input: &str) -> u64 {
    let mut monkies: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey| {
            let lines: Vec<&str> = monkey.lines().collect();

            Monkey {
                items: lines
                    .iter()
                    .find(|l| l.contains("Starting items:"))
                    .unwrap()
                    .trim()
                    .strip_prefix("Starting items: ")
                    .unwrap()
                    .split(',')
                    .map(|num| num.trim().parse().unwrap())
                    .collect(),
                operation: Operation::from_lines(&lines).unwrap(),
                test: Test::from_lines(&lines).unwrap(),
                num_inspections: 0,
            }
        })
        .collect();

    let common_divisor: u64 = monkies.iter().map(|m| m.test.divisor).product();

    for _ in 0..10000 {
        let mut monkey_index = 0;
        while monkey_index < monkies.len() {
            while let Some(item) = monkies[monkey_index].items.pop_front() {
                monkies[monkey_index].num_inspections += 1;
                let new_item = monkies[monkey_index].operation.run(item) % common_divisor;
                let new_monkey_index = monkies[monkey_index].test.run(new_item);
                monkies[new_monkey_index].items.push_back(new_item);
            }
            monkey_index += 1;
        }
    }

    let mut num_inspections: Vec<u64> = monkies.into_iter().map(|m| m.num_inspections).collect();
    num_inspections.sort();

    num_inspections.pop().unwrap() * num_inspections.pop().unwrap()
}
