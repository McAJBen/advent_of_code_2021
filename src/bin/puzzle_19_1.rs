use std::{
    collections::{HashMap, HashSet},
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

struct Offset {
    rotation_index: usize,
    diff: Beacon,
}

#[derive(Debug)]
struct Scanner {
    beacons: HashSet<Beacon>,
}

impl Scanner {
    fn get_offset(&self, other: &Scanner) -> Option<Offset> {
        for rotation_index in 0..24 {
            let beacons = other.rotate(rotation_index);

            let mut diffs = HashMap::new();

            for beacon in beacons.iter() {
                for self_beacon in self.beacons.iter() {
                    let diff = beacon.clone() - self_beacon.clone();
                    let entry = diffs.entry(diff).or_insert(0);
                    *entry += 1;

                    if *entry >= 12 {
                        break;
                    }
                }
            }

            let solution = diffs.into_iter().find(|(_, count)| *count >= 12);
            if let Some(solution) = solution {
                return Some(Offset {
                    rotation_index,
                    diff: solution.0,
                });
            }
        }

        None
    }

    fn add_beacons(&mut self, other: &Scanner, offset: &Offset) {
        let beacons = &other.rotate(offset.rotation_index);

        for beacon in beacons.iter() {
            self.beacons.insert(beacon.clone() - offset.diff.clone());
        }
    }

    fn rotate(&self, rotation: usize) -> Vec<Beacon> {
        self.beacons.iter().map(|b| b.rotate(rotation)).collect()
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

    while let Some((scanner_index, offset)) = scanners
        .iter()
        .enumerate()
        .find_map(|(index, s)| base_scanner.get_offset(s).map(|x| (index, x)))
    {
        let scanner = scanners.remove(scanner_index);

        base_scanner.add_beacons(&scanner, &offset);
    }

    assert!(scanners.is_empty());

    let num_beacons = base_scanner.beacons.len();

    assert_eq!(362, num_beacons);

    println!("{}", num_beacons);
}
