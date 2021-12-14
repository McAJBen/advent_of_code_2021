use advent_of_code::Point;
use std::{collections::HashSet, fs::read_to_string};

fn display_grid(points: &HashSet<Point>) {
    let width = points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = points.iter().map(|p| p.y).max().unwrap() + 1;

    for y in 0..height {
        for x in 0..width {
            let point = Point { x, y };
            if points.contains(&point) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let input = read_to_string("puzzle_13_input").unwrap();

    let (points, folds) = input.split_once("\n\n").unwrap();

    let mut points = points.lines().map(Point::from_str).collect::<HashSet<_>>();

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
