use crate::utils::{read_input, ZipWithNextExt};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part1(input: &str) -> usize {
    let orders: Vec<Direction> = input
        .lines()
        .flat_map(|line| {
            let (direction, count) = line.split_once(' ').unwrap();
            let direction = match direction {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!(),
            };
            let count = usize::from_str_radix(count, 10).unwrap();

            vec![direction; count]
        })
        .collect();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut tail_history = HashSet::new();
    tail_history.insert(tail.clone());

    for direction in orders {
        match direction {
            Direction::Up => head.1 -= 1,
            Direction::Down => head.1 += 1,
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
        }

        match (head.0 - tail.0, head.1 - tail.1) {
            (-2, 0) => tail.0 -= 1,
            (2, 0) => tail.0 += 1,
            (0, -2) => tail.1 -= 1,
            (0, 2) => tail.1 += 1,

            (-2, -1) | (-1, -2) => {
                tail.0 -= 1;
                tail.1 -= 1
            }
            (-2, 1) | (-1, 2) => {
                tail.0 -= 1;
                tail.1 += 1;
            }
            (1, -2) | (2, -1) => {
                tail.0 += 1;
                tail.1 -= 1;
            }
            (2, 1) | (1, 2) => {
                tail.0 += 1;
                tail.1 += 1;
            }
            _ => {}
        }

        tail_history.insert(tail.clone());
    }

    tail_history.len()
}

#[test]
fn test_part1() {
    let input = read_input(2022, 9);
    assert_eq!(part1(&input), 5883);
}

pub fn part2(input: &str) -> usize {
    let orders: Vec<Direction> = input
        .lines()
        .flat_map(|line| {
            let (direction, count) = line.split_once(' ').unwrap();
            let direction = match direction {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!(),
            };
            let count = usize::from_str_radix(count, 10).unwrap();

            vec![direction; count]
        })
        .collect();

    let mut snake = vec![(0, 0); 10];

    let mut tail_history = HashSet::new();
    tail_history.insert(snake.last().unwrap().clone());

    for direction in orders {
        {
            let head = snake.first_mut().unwrap();
            match direction {
                Direction::Up => head.1 -= 1,
                Direction::Down => head.1 += 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
            }
        }

        for (head, tail) in (0..snake.len()).zip_with_next() {
            let head = snake[head];
            let tail = &mut snake[tail];
            match (head.0 - tail.0, head.1 - tail.1) {
                (-2, 0) => tail.0 -= 1,
                (2, 0) => tail.0 += 1,
                (0, -2) => tail.1 -= 1,
                (0, 2) => tail.1 += 1,

                (-2, -1) | (-1, -2) | (-2, -2) => {
                    tail.0 -= 1;
                    tail.1 -= 1
                }
                (-2, 1) | (-1, 2) | (-2, 2) => {
                    tail.0 -= 1;
                    tail.1 += 1;
                }
                (1, -2) | (2, -1) | (2, -2) => {
                    tail.0 += 1;
                    tail.1 -= 1;
                }
                (2, 1) | (1, 2) | (2, 2) => {
                    tail.0 += 1;
                    tail.1 += 1;
                }
                _ => {}
            }
        }

        tail_history.insert(snake.last().unwrap().clone());
    }

    tail_history.len()
}

#[test]
fn test_part2() {
    let input = read_input(2022, 9);
    assert_eq!(part2(&input), 0);
}
