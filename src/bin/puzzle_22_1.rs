use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

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
        let x = (x.0.parse::<i32>().unwrap(), x.1.parse::<i32>().unwrap());

        let y = dimensions[1][2..].split_once("..").unwrap();
        let y = (y.0.parse::<i32>().unwrap(), y.1.parse::<i32>().unwrap());

        let z = dimensions[2][2..].split_once("..").unwrap();
        let z = (z.0.parse::<i32>().unwrap(), z.1.parse::<i32>().unwrap());

        Self {
            set_type: set_type == "on",
            x,
            y,
            z,
        }
    }

    fn get_coordinates(&self) -> Vec<Coordinate> {
        (self.x.0..=self.x.1)
            .flat_map(|x| {
                (self.y.0..=self.y.1)
                    .flat_map(move |y| (self.z.0..=self.z.1).map(move |z| Coordinate { x, y, z }))
            })
            .collect()
    }
}

fn main() {
    let input = read_to_string("input/22").unwrap();

    let mut lit_cells = HashSet::new();

    for line in input.lines() {
        let cuboid = Cuboid::new(line);

        if cuboid.x.0 < -50
            || cuboid.x.1 > 50
            || cuboid.y.0 < -50
            || cuboid.y.1 > 50
            || cuboid.z.0 < -50
            || cuboid.z.1 > 50
        {
            continue;
        }

        if cuboid.set_type {
            lit_cells.extend(cuboid.get_coordinates());
        } else {
            for coordinate in cuboid.get_coordinates() {
                lit_cells.remove(&coordinate);
            }
        }
    }

    let num_lit = lit_cells.len();

    assert_eq!(644257, num_lit);

    println!("{}", num_lit);
}
