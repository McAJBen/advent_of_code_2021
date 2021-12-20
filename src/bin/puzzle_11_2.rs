use advent_of_code_2021::Point;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Octopus {
    position: Point,
    energy: u8,
    neighbors: Vec<Point>,
}

#[derive(Debug, Clone)]
struct OctopusGrid {
    octopuses: Vec<Octopus>,
}

impl OctopusGrid {
    fn new(input: String) -> Self {
        let octopus_energies: Vec<Vec<u8>> = input
            .lines()
            .into_iter()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        let width = octopus_energies[0].len();
        let height = octopus_energies.len();

        let mut octopuses = Vec::new();

        for (y, row) in octopus_energies.into_iter().enumerate() {
            for (x, energy) in row.into_iter().enumerate() {
                let neighbors = [
                    (0, 0),
                    (0, 1),
                    (0, 2),
                    (1, 0),
                    (1, 2),
                    (2, 0),
                    (2, 1),
                    (2, 2),
                ]
                .into_iter()
                .filter_map(|(offset_x, offset_y)| {
                    let neighbor_x = x + offset_x;
                    let neighbor_y = y + offset_y;

                    if neighbor_x >= 1
                        && neighbor_x <= width
                        && neighbor_y >= 1
                        && neighbor_y <= height
                    {
                        Some(Point {
                            x: neighbor_x - 1,
                            y: neighbor_y - 1,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

                octopuses.push(Octopus {
                    position: Point { x, y },
                    energy,
                    neighbors,
                });
            }
        }

        Self { octopuses }
    }

    fn tick(&mut self) -> u32 {
        let mut energy_increases = self
            .octopuses
            .iter()
            .map(|octopus| octopus.position)
            .collect::<Vec<_>>();

        let mut num_flashes = 0;

        while let Some(octopus_position) = energy_increases.pop() {
            let octopus = self
                .octopuses
                .iter_mut()
                .find(|octopus| octopus.position == octopus_position)
                .unwrap();

            octopus.energy += 1;

            if octopus.energy == 10 {
                num_flashes += 1;
                energy_increases.extend(octopus.neighbors.clone());
            }
        }

        for octopus in self.octopuses.iter_mut() {
            if octopus.energy >= 10 {
                octopus.energy = 0;
            }
        }

        num_flashes
    }
}

fn main() {
    let input = read_to_string("input/11").unwrap();

    let mut grid = OctopusGrid::new(input);

    let mut num_ticks = 0;
    loop {
        num_ticks += 1;

        if grid.tick() == 100 {
            break;
        }
    }

    assert_eq!(216, num_ticks);

    println!("{}", num_ticks);
}
