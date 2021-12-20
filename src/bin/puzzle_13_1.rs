use advent_of_code_2021::Point;
use std::{collections::HashSet, fs::read_to_string, str::FromStr};

fn main() {
    let input = read_to_string("input/13").unwrap();

    let (points, folds) = input.split_once("\n\n").unwrap();

    let mut points = points
        .lines()
        .map(|line| Point::from_str(line).unwrap())
        .collect::<HashSet<_>>();

    for line in folds.lines().take(1) {
        let (axis, value) = line[11..].split_once('=').unwrap();

        let value = value.parse::<usize>().unwrap();

        points = points
            .into_iter()
            .map(|point| {
                let Point { x, y } = point;
                match axis {
                    "x" => {
                        if x > value {
                            Point {
                                x: 2 * value - x,
                                y,
                            }
                        } else {
                            Point { x, y }
                        }
                    }
                    "y" => {
                        if y > value {
                            Point {
                                x,
                                y: 2 * value - y,
                            }
                        } else {
                            Point { x, y }
                        }
                    }
                    _ => panic!("Unknown axis: {}", axis),
                }
            })
            .collect();
    }

    let num_points = points.len();

    assert_eq!(671, num_points);

    println!("{:#?}", num_points);
}
