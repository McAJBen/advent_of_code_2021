use advent_of_code_2021::ZipWithNextExt;
use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone)]
struct Cuboid {
    set_type: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Cuboid {
    fn new(input: &str) -> Self {
        // on x=-20..33,y=-21..23,z=-26..28
        let (set_type, rest) = input.split_once(' ').unwrap();
        let dimensions = rest.split(',').collect::<Vec<_>>();

        let x = dimensions[0][2..].split_once("..").unwrap();
        let x = (x.0.parse::<i32>().unwrap(), x.1.parse::<i32>().unwrap() + 1);

        let y = dimensions[1][2..].split_once("..").unwrap();
        let y = (y.0.parse::<i32>().unwrap(), y.1.parse::<i32>().unwrap() + 1);

        let z = dimensions[2][2..].split_once("..").unwrap();
        let z = (z.0.parse::<i32>().unwrap(), z.1.parse::<i32>().unwrap() + 1);

        Self {
            set_type: set_type == "on",
            x,
            y,
            z,
        }
    }
}

fn main() {
    let input = read_to_string("input/22").unwrap();

    let cuboids = input.lines().map(Cuboid::new).collect::<Vec<_>>();

    let mut x_axis = cuboids
        .iter()
        .flat_map(|c| [c.x.0, c.x.1])
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    x_axis.sort_unstable();
    let x_axis = x_axis.into_iter().zip_with_next().collect::<Vec<_>>();

    let mut y_axis = cuboids
        .iter()
        .flat_map(|c| [c.y.0, c.y.1])
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    y_axis.sort_unstable();
    let y_axis = y_axis.into_iter().zip_with_next().collect::<Vec<_>>();

    let mut z_axis = cuboids
        .iter()
        .flat_map(|c| [c.z.0, c.z.1])
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    z_axis.sort_unstable();
    let z_axis = z_axis.into_iter().zip_with_next().collect::<Vec<_>>();

    let mut cubes = vec![vec![vec![false; x_axis.len()]; y_axis.len()]; z_axis.len()];

    for cuboid in cuboids {
        let x_ranges = x_axis
            .iter()
            .enumerate()
            .filter(|(_, &x)| cuboid.x.0 <= x.0 && x.0 < cuboid.x.1)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let y_ranges = y_axis
            .iter()
            .enumerate()
            .filter(|(_, &y)| cuboid.y.0 <= y.0 && y.0 < cuboid.y.1)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let z_ranges = z_axis
            .iter()
            .enumerate()
            .filter(|(_, &z)| cuboid.z.0 <= z.0 && z.0 < cuboid.z.1)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        for x_range in x_ranges {
            for &y_range in y_ranges.iter() {
                for &z_range in z_ranges.iter() {
                    if cuboid.set_type {
                        cubes[z_range][y_range][x_range] = true;
                    } else {
                        cubes[z_range][y_range][x_range] = false;
                    }
                }
            }
        }
    }

    let mut num_lit: u64 = 0;

    for (index_x, x) in x_axis.iter().enumerate() {
        for (index_y, y) in y_axis.iter().enumerate() {
            for (index_z, z) in z_axis.iter().enumerate() {
                if cubes[index_z][index_y][index_x] {
                    num_lit += (x.1 - x.0) as u64 * (y.1 - y.0) as u64 * (z.1 - z.0) as u64;
                }
            }
        }
    }

    assert_eq!(1235484513229032, num_lit);

    println!("{}", num_lit);
}
