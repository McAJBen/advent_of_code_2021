use advent_of_code::read_lines;
use std::{collections::HashSet, fs::File};

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    z_heights: Vec<Vec<u8>>,
}

impl Grid {
    fn new(lines: Vec<String>) -> Self {
        let z_heights: Vec<Vec<u8>> = lines
            .into_iter()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        let width = z_heights[0].len();
        let height = z_heights.len();

        assert!(width >= 2);
        assert!(height >= 2);

        Self {
            width,
            height,
            z_heights,
        }
    }

    fn get_nearby_heights(&self) -> Vec<((usize, usize), HashSet<(usize, usize)>)> {
        let mut heights = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut nearby_heights = HashSet::new();
                if x < self.width - 1 {
                    nearby_heights.insert((x + 1, y));
                }
                if x > 0 {
                    nearby_heights.insert((x - 1, y));
                }
                if y < self.height - 1 {
                    nearby_heights.insert((x, y + 1));
                }
                if y > 0 {
                    nearby_heights.insert((x, y - 1));
                }
                heights.push(((x, y), nearby_heights));
            }
        }
        heights
    }

    fn get_low_points(&self) -> Vec<u8> {
        let nearby_heights = self.get_nearby_heights();
        nearby_heights
            .into_iter()
            .filter_map(|(center, nearby)| {
                let center = self.z_heights[center.1][center.0];
                if nearby.iter().all(|neighbor| {
                    let neighbor = self.z_heights[neighbor.1][neighbor.0];
                    neighbor > center
                }) {
                    Some(center)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}

fn main() {
    let input = File::open("puzzle_9_input").unwrap();

    let grid = Grid::new(read_lines(&input));

    let low_points = grid.get_low_points();

    println!("{:?}", low_points);

    let total = low_points.into_iter().map(|x| x as u128 + 1).sum::<u128>();

    println!("{:?}", total);
}
