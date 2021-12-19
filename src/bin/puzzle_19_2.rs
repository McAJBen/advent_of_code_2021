use std::{
    collections::HashSet,
    fs::read_to_string,
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

    fn rotations(&self) -> [Beacon; 24] {
        let Beacon { x, y, z } = self.clone();
        [
            Beacon::new(x, y, z),
            Beacon::new(-x, -y, z),
            Beacon::new(-x, y, -z),
            Beacon::new(x, -y, -z),
            Beacon::new(-x, -z, -y),
            Beacon::new(-x, z, y),
            Beacon::new(x, -z, y),
            Beacon::new(x, z, -y),
            Beacon::new(-y, x, z),
            Beacon::new(y, -x, z),
            Beacon::new(y, x, -z),
            Beacon::new(-y, -x, -z),
            Beacon::new(y, z, x),
            Beacon::new(-y, -z, x),
            Beacon::new(-y, z, -x),
            Beacon::new(y, -z, -x),
            Beacon::new(z, x, y),
            Beacon::new(-z, -x, y),
            Beacon::new(-z, x, -y),
            Beacon::new(z, -x, -y),
            Beacon::new(-z, y, x),
            Beacon::new(z, -y, x),
            Beacon::new(z, y, -x),
            Beacon::new(-z, -y, -x),
        ]
    }

    fn manhattan_distance(&self, other: &Beacon) -> i16 {
        let dist = (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs();
        dist
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
    fn get_offset(&self, other: &Scanner) -> Option<(usize, Beacon)> {
        for (rotation_index, rotation) in other.rotations().iter().enumerate() {
            for beacon in rotation.beacons.iter() {
                for self_beacon in self.beacons.iter() {
                    let diff = beacon.clone() - self_beacon.clone();

                    let num_matching_becaons = self
                        .beacons
                        .iter()
                        .map(|b| b.clone() + diff.clone())
                        .filter(|b| rotation.beacons.contains(b))
                        .count();

                    if num_matching_becaons >= 12 {
                        return Some((rotation_index, diff));
                    }
                }
            }
        }

        None
    }

    fn matches_with(&self, other: &Scanner) -> bool {
        self.get_offset(other).is_some()
    }

    fn add_beacons(&mut self, other: &Scanner) -> Beacon {
        let (rotation_index, diff) = self.get_offset(other).unwrap();

        let rotation = &other.rotations()[rotation_index];

        for beacon in rotation.beacons.iter() {
            self.beacons.insert(beacon.clone() - diff.clone());
        }

        diff
    }

    fn rotations(&self) -> Vec<Scanner> {
        let beacons = self
            .beacons
            .iter()
            .map(|b| b.rotations())
            .collect::<Vec<_>>();

        (0..24)
            .map(|beacon_group| Scanner {
                beacons: beacons.iter().map(|b| b[beacon_group].clone()).collect(),
            })
            .collect()
    }
}

fn main() {
    let input = read_to_string("puzzle_19_input").unwrap();

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

    while let Some((scanner_index, _)) = scanners
        .iter()
        .enumerate()
        .find(|(_, s)| base_scanner.matches_with(s))
    {
        let scanner = scanners.remove(scanner_index);

        let scanner_position = base_scanner.add_beacons(&scanner);
        scanner_positions.push(scanner_position);
    }

    assert!(scanners.is_empty());

    let max_distance = scanner_positions
        .iter()
        .enumerate()
        .flat_map(|(index, beacon1)| {
            scanner_positions
                .iter()
                .skip(index + 1)
                .map(|beacon2| beacon1.manhattan_distance(beacon2))
        })
        .max()
        .unwrap();

    assert_eq!(12204, max_distance);

    println!("{}", max_distance);
}
