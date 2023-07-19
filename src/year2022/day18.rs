use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    X,
    Y,
    Z,
}

fn parse_line(line: &str) -> (u8, u8, u8) {
    let (x, yz) = line.split_once(',').unwrap();
    let (y, z) = yz.split_once(',').unwrap();
    let x = u8::from_str_radix(x, 10).unwrap();
    let y = u8::from_str_radix(y, 10).unwrap();
    let z = u8::from_str_radix(z, 10).unwrap();
    (x, y, z)
}

pub fn part1(input: &str) -> usize {
    let mut sides = HashSet::<(Side, u8, u8, u8)>::new();

    for line in input.lines() {
        let (x, y, z) = parse_line(line);
        let new_sides = [
            (Side::X, x, y, z),
            (Side::Y, x, y, z),
            (Side::Z, x, y, z),
            (Side::X, x + 1, y, z),
            (Side::Y, x, y + 1, z),
            (Side::Z, x, y, z + 1),
        ];

        for s in new_sides {
            if !sides.remove(&s) {
                sides.insert(s);
            }
        }
    }

    sides.len()
}

pub fn part2(input: &str) -> usize {
    let points: HashSet<(u8, u8, u8)> = input
        .lines()
        .map(|line| parse_line(line))
        .map(|(x, y, z)| (x + 1, y + 1, z + 1))
        .collect();

    let max_x = *points.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let max_y = *points.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let max_z = *points.iter().map(|(_, _, z)| z).max().unwrap() + 1;

    let mut num_sides = 0;

    let mut to_be_tested: VecDeque<(u8, u8, u8)> = [(0, 0, 0)].into_iter().collect();
    let mut visited = HashSet::new();

    while let Some((x, y, z)) = to_be_tested.pop_front() {
        if !visited.insert((x, y, z)) {
            continue;
        }

        if x > 0 {
            if points.contains(&(x - 1, y, z)) {
                num_sides += 1;
            } else {
                to_be_tested.push_back((x - 1, y, z));
            }
        }
        if x < max_x {
            if points.contains(&(x + 1, y, z)) {
                num_sides += 1;
            } else {
                to_be_tested.push_back((x + 1, y, z));
            }
        }
        if y > 0 {
            if points.contains(&(x, y - 1, z)) {
                num_sides += 1;
            } else {
                to_be_tested.push_back((x, y - 1, z));
            }
        }
        if y < max_y {
            if points.contains(&(x, y + 1, z)) {
                num_sides += 1;
            } else {
                to_be_tested.push_back((x, y + 1, z));
            }
        }
        if z > 0 {
            if points.contains(&(x, y, z - 1)) {
                num_sides += 1;
            } else {
                to_be_tested.push_back((x, y, z - 1));
            }
        }
        if z < max_z {
            if points.contains(&(x, y, z + 1)) {
                num_sides += 1;
            } else {
                to_be_tested.push_back((x, y, z + 1));
            }
        }
    }

    num_sides
}
