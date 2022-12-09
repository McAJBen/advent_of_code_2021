use crate::utils::read_input;
use std::ops::Add;

#[derive(Debug, Clone)]
struct Cell {
    depth: u8,
    value: u32,
}

#[derive(Debug, Clone)]
struct SnailfishNumber {
    values: Vec<Cell>,
}

impl SnailfishNumber {
    fn new(input: &str) -> Self {
        let mut depth = 0u8;
        let mut values = Vec::new();
        for c in input.chars() {
            if c == '[' {
                depth += 1;
            } else if c == ']' {
                depth -= 1;
            } else if c != ',' {
                values.push(Cell {
                    depth,
                    value: c.to_digit(10).unwrap(),
                });
            }
        }
        Self { values }
    }

    fn reduce(&mut self) {
        loop {
            if let Some((index, _)) = self
                .values
                .iter()
                .enumerate()
                .find(|(_, cell)| cell.depth == 5)
            {
                let left = self.values.remove(index);

                if index > 0 {
                    self.values[index - 1].value += left.value;
                }
                if index + 1 < self.values.len() {
                    self.values[index + 1].value += self.values[index].value;
                }

                self.values[index].depth -= 1;
                self.values[index].value = 0;

                continue;
            }

            if let Some((index, _)) = self
                .values
                .iter()
                .enumerate()
                .find(|(_, cell)| cell.value >= 10)
            {
                let depth = self.values[index].depth + 1;
                let value = self.values[index].value;

                self.values.insert(
                    index,
                    Cell {
                        depth,
                        value: value / 2,
                    },
                );
                self.values[index + 1].depth = depth;
                self.values[index + 1].value = value - (value / 2);

                continue;
            }

            break;
        }
    }

    fn magnitude(&self) -> u32 {
        let mut values = self.values.clone();

        for depth in (1..=5).rev() {
            while let Some((index, _)) = values
                .iter()
                .enumerate()
                .find(|(_, cell)| cell.depth == depth)
            {
                let left = values.remove(index);

                values[index].depth -= 1;
                values[index].value = 3 * left.value + 2 * values[index].value;
            }
        }

        values[0].value
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = SnailfishNumber {
            values: self
                .values
                .iter()
                .map(|cell| Cell {
                    depth: cell.depth + 1,
                    value: cell.value,
                })
                .chain(other.values.iter().map(|cell| Cell {
                    depth: cell.depth + 1,
                    value: cell.value,
                }))
                .collect(),
        };

        result.reduce();

        result
    }
}

pub fn part1(input: &str) -> u32 {
    let result = input
        .lines()
        .map(SnailfishNumber::new)
        .reduce(|a, b| a + b)
        .unwrap();

    result.magnitude()
}

#[test]
fn test_part1() {
    let input = read_input(2021, 18);
    assert_eq!(part1(&input), 4008);
}

pub fn part2(input: &str) -> u32 {
    let snailfish_numbers = input.lines().map(SnailfishNumber::new).collect::<Vec<_>>();

    let mut max_magnitude = 0;

    for (index_a, number_a) in snailfish_numbers.iter().enumerate() {
        for (index_b, number_b) in snailfish_numbers.iter().enumerate() {
            if index_a == index_b {
                continue;
            }
            let result = (number_a.clone() + number_b.clone()).magnitude();
            max_magnitude = max_magnitude.max(result);
        }
    }

    max_magnitude
}

#[test]
fn test_part2() {
    let input = read_input(2021, 18);
    assert_eq!(part2(&input), 4667);
}
