use advent_of_code::read_lines;
use std::{collections::HashMap, fs::File};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn new(line: &String) -> Self {
        let (point1, point2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = point1.split_once(",").unwrap();
        let (x2, y2) = point2.split_once(",").unwrap();

        Self {
            x1: x1.parse().unwrap(),
            x2: x2.parse().unwrap(),
            y1: y1.parse().unwrap(),
            y2: y2.parse().unwrap(),
        }
    }
}

fn main() {
    let input = File::open("puzzle_5_input").unwrap();

    let lines = read_lines(&input);

    let lines = lines
        .into_iter()
        .map(|line| Line::new(&line))
        .collect::<Vec<_>>();

    let size = lines
        .iter()
        .map(|line| {
            [line.x1, line.y1, line.x2, line.y2]
                .into_iter()
                .max()
                .unwrap()
        })
        .max()
        .unwrap() as usize
        + 1;

    println!("{}", size);

    let mut map = HashMap::new();

    for line in lines {
        let Line { x1, y1, x2, y2 } = line;

        if x1 < x2 {
            if y1 == y2 {
                for x in x1..=x2 {
                    *map.entry((x, y1)).or_insert(0) += 1;
                }
            }
        } else if x1 == x2 {
            if y1 < y2 {
                for y in y1..=y2 {
                    *map.entry((x1, y)).or_insert(0) += 1;
                }
            } else if y1 == y2 {
                *map.entry((x1, y1)).or_insert(0) += 1;
            } else {
                for y in y2..=y1 {
                    *map.entry((x1, y)).or_insert(0) += 1;
                }
            }
        } else {
            if y1 == y2 {
                for x in x2..=x1 {
                    *map.entry((x, y1)).or_insert(0) += 1;
                }
            }
        }
    }

    let num_2 = map.iter().filter(|(_, &x)| x > 1).count();

    // for row in grid {
    //     for x in row {
    //         print!("{}", x);
    //     }
    //     println!();
    // }

    println!("{}", num_2);
}
