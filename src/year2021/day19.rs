use crate::utils::read_input;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beacon {
    x: i16,
    y: i16,
    z: i16,
}

impl Beacon {
    fn new(x: i16, y: i16, z: i16) -> Beacon {
        Beacon { x, y, z }
    }

    fn rotate(&self, rotation: usize) -> Beacon {
        let Beacon { x, y, z } = self.clone();
        match rotation {
            0 => Beacon::new(x, y, z),
            1 => Beacon::new(-x, -y, z),
            2 => Beacon::new(-x, y, -z),
            3 => Beacon::new(x, -y, -z),
            4 => Beacon::new(-x, -z, -y),
            5 => Beacon::new(-x, z, y),
            6 => Beacon::new(x, -z, y),
            7 => Beacon::new(x, z, -y),
            8 => Beacon::new(-y, x, z),
            9 => Beacon::new(y, -x, z),
            10 => Beacon::new(y, x, -z),
            11 => Beacon::new(-y, -x, -z),
            12 => Beacon::new(y, z, x),
            13 => Beacon::new(-y, -z, x),
            14 => Beacon::new(-y, z, -x),
            15 => Beacon::new(y, -z, -x),
            16 => Beacon::new(z, x, y),
            17 => Beacon::new(-z, -x, y),
            18 => Beacon::new(-z, x, -y),
            19 => Beacon::new(z, -x, -y),
            20 => Beacon::new(-z, y, x),
            21 => Beacon::new(z, -y, x),
            22 => Beacon::new(z, y, -x),
            23 => Beacon::new(-z, -y, -x),
            _ => panic!("rotation out of range"),
        }
    }

    fn manhattan_distance(&self, other: &Beacon) -> i16 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Sub for Beacon {
    type Output = Beacon;

    fn sub(self, other: Beacon) -> Beacon {
        Beacon::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Add for Beacon {
    type Output = Beacon;

    fn add(self, other: Beacon) -> Beacon {
        Beacon::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

#[derive(Debug)]
struct Scanner {
    beacons: HashSet<Beacon>,
}

impl Scanner {
    fn check_merge(&mut self, other: &Scanner) -> Option<Beacon> {
        for rotation_index in 0..24 {
            let beacons = other
                .beacons
                .iter()
                .map(|b| b.rotate(rotation_index))
                .collect::<Vec<_>>();

            let mut diffs = HashMap::new();

            for beacon in beacons.iter() {
                for self_beacon in self.beacons.iter() {
                    let diff = beacon.clone() - self_beacon.clone();
                    let entry = diffs.entry(diff.clone()).or_insert(0);
                    *entry += 1;

                    if *entry >= 12 {
                        self.beacons
                            .extend(beacons.into_iter().map(|b| b - diff.clone()));
                        return Some(diff);
                    }
                }
            }
        }

        None
    }
}

pub fn part1() -> usize {
    let input = read_input(2021, 19);

    let mut scanners = Vec::new();
    for line in input.lines() {
        if line.starts_with("---") {
            scanners.push(Scanner {
                beacons: HashSet::new(),
            });
        } else if !line.is_empty() {
            let points = line
                .splitn(3, ',')
                .map(|num| num.parse::<i16>().unwrap())
                .collect::<Vec<_>>();

            scanners.last_mut().unwrap().beacons.insert(Beacon {
                x: points[0],
                y: points[1],
                z: points[2],
            });
        }
    }

    let mut base_scanner = scanners.pop().unwrap();

    while let Some((scanner_index, _)) = scanners
        .iter()
        .enumerate()
        .find(|(_, s)| base_scanner.check_merge(s).is_some())
    {
        scanners.swap_remove(scanner_index);
    }

    debug_assert!(scanners.is_empty());

    base_scanner.beacons.len()
}

pub fn part2() -> i16 {
    let input = read_input(2021, 19);

    let mut scanners = Vec::new();
    for line in input.lines() {
        if line.starts_with("---") {
            scanners.push(Scanner {
                beacons: HashSet::new(),
            });
        } else if !line.is_empty() {
            let points = line
                .splitn(3, ',')
                .map(|num| num.parse::<i16>().unwrap())
                .collect::<Vec<_>>();

            scanners.last_mut().unwrap().beacons.insert(Beacon {
                x: points[0],
                y: points[1],
                z: points[2],
            });
        }
    }

    let mut base_scanner = scanners.remove(0);

    let mut scanner_positions = vec![Beacon::new(0, 0, 0)];

    while let Some((scanner_index, offset)) = scanners
        .iter()
        .enumerate()
        .find_map(|(index, s)| base_scanner.check_merge(s).map(|offset| (index, offset)))
    {
        scanners.swap_remove(scanner_index);
        scanner_positions.push(offset);
    }

    debug_assert!(scanners.is_empty());

    scanner_positions
        .iter()
        .enumerate()
        .flat_map(|(index, beacon1)| {
            scanner_positions
                .iter()
                .skip(index + 1)
                .map(|beacon2| beacon1.manhattan_distance(beacon2))
        })
        .max()
        .unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 362);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 12204);
}
