use crate::utils::{read_input, Point};
use std::{collections::HashSet, str::FromStr};

fn format_grid(points: &HashSet<Point>) -> String {
    let width = points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = points.iter().map(|p| p.y).max().unwrap() + 1;

    let mut s = String::new();

    for y in 0..height {
        for x in 0..width {
            let point = Point { x, y };
            if points.contains(&point) {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }

    s
}

pub fn part1() -> usize {
    let input = read_input(2021, 13);

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

    points.len()
}

pub fn part2() -> String {
    let input = read_input(2021, 13);

    let (points, folds) = input.split_once("\n\n").unwrap();

    let mut points = points
        .lines()
        .map(|line| Point::from_str(line).unwrap())
        .collect::<HashSet<_>>();

    for line in folds.lines() {
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

    format_grid(&points)
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 671);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), "###   ##  ###  #  #  ##  ###  #  # #   \n#  # #  # #  # #  # #  # #  # # #  #   \n#  # #    #  # #### #  # #  # ##   #   \n###  #    ###  #  # #### ###  # #  #   \n#    #  # #    #  # #  # # #  # #  #   \n#     ##  #    #  # #  # #  # #  # ####\n");
}
