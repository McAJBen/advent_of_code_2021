use std::{fs::read_to_string, ops::Add};

#[derive(Debug, Clone)]
struct Cell {
    depth: u8,
    value: u32,
}

#[derive(Debug)]
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

fn main() {
    let input = read_to_string("input/18").unwrap();

    let result = input
        .lines()
        .map(SnailfishNumber::new)
        .reduce(|a, b| a + b)
        .unwrap();

    let magnitude = result.magnitude();

    assert_eq!(4008, magnitude);

    println!("{}", magnitude);
}
