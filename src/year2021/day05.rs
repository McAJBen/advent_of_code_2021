use crate::utils::read_input;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn new(line: &str) -> Self {
        let (point1, point2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = point1.split_once(',').unwrap();
        let (x2, y2) = point2.split_once(',').unwrap();

        Self {
            x1: x1.parse().unwrap(),
            x2: x2.parse().unwrap(),
            y1: y1.parse().unwrap(),
            y2: y2.parse().unwrap(),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let lines = input.lines().map(Line::new).collect::<Vec<_>>();

    let mut map = HashMap::new();

    for line in lines {
        let Line { x1, y1, x2, y2 } = line;

        match x1.cmp(&x2) {
            Ordering::Less => {
                if y1 == y2 {
                    for x in x1..=x2 {
                        *map.entry((x, y1)).or_insert(0) += 1;
                    }
                }
            }
            Ordering::Equal => {
                match y1.cmp(&y2) {
                    Ordering::Less => {
                        for y in y1..=y2 {
                            *map.entry((x1, y)).or_insert(0) += 1;
                        }
                    }
                    Ordering::Equal => {
                        *map.entry((x1, y1)).or_insert(0) += 1;
                    }
                    Ordering::Greater => {
                        for y in y2..=y1 {
                            *map.entry((x1, y)).or_insert(0) += 1;
                        }
                    }
                };
            }
            Ordering::Greater => {
                if y1 == y2 {
                    for x in x2..=x1 {
                        *map.entry((x, y1)).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    map.iter().filter(|(_, &x)| x > 1).count()
}

#[test]
fn test_part1() {
    let input = read_input(2021, 5);
    assert_eq!(part1(&input), 4826);
}

pub fn part2(input: &str) -> usize {
    let lines = input.lines().map(Line::new).collect::<Vec<_>>();

    let mut map = HashMap::new();

    for line in lines {
        let Line { x1, y1, x2, y2 } = line;

        match x1.cmp(&x2) {
            Ordering::Less => match y1.cmp(&y2) {
                Ordering::Less => {
                    for offset in 0..=(x2 - x1) {
                        *map.entry((x1 + offset, y1 + offset)).or_insert(0) += 1;
                    }
                }
                Ordering::Equal => {
                    for x in x1..=x2 {
                        *map.entry((x, y1)).or_insert(0) += 1;
                    }
                }
                Ordering::Greater => {
                    for offset in 0..=(x2 - x1) {
                        *map.entry((x1 + offset, y1 - offset)).or_insert(0) += 1;
                    }
                }
            },
            Ordering::Equal => match y1.cmp(&y2) {
                Ordering::Less => {
                    for y in y1..=y2 {
                        *map.entry((x1, y)).or_insert(0) += 1;
                    }
                }
                Ordering::Equal => {
                    *map.entry((x1, y1)).or_insert(0) += 1;
                }
                Ordering::Greater => {
                    for y in y2..=y1 {
                        *map.entry((x1, y)).or_insert(0) += 1;
                    }
                }
            },
            Ordering::Greater => match y1.cmp(&y2) {
                Ordering::Less => {
                    for offset in 0..=(x1 - x2) {
                        *map.entry((x1 - offset, y1 + offset)).or_insert(0) += 1;
                    }
                }
                Ordering::Equal => {
                    for x in x2..=x1 {
                        *map.entry((x, y1)).or_insert(0) += 1;
                    }
                }
                Ordering::Greater => {
                    for offset in 0..=(x1 - x2) {
                        *map.entry((x1 - offset, y1 - offset)).or_insert(0) += 1;
                    }
                }
            },
        }
    }

    map.iter().filter(|(_, &x)| x > 1).count()
}

#[test]
fn test_part2() {
    let input = read_input(2021, 5);
    assert_eq!(part2(&input), 16793);
}
