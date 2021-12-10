use advent_of_code::read_lines;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

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

    fn get_nearby(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
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
        nearby_heights
    }

    fn get_nearby_heights(&self) -> Vec<((usize, usize), HashSet<(usize, usize)>)> {
        let mut heights = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                heights.push(((x, y), self.get_nearby(x, y)));
            }
        }
        heights
    }

    fn get_basin_size(&self, x: usize, y: usize) -> usize {
        let mut processed_points = HashSet::new();
        let mut unprocessed_points = vec![(x, y)];

        while let Some(point) = unprocessed_points.pop() {
            if processed_points.contains(&point) {
                continue;
            }

            let height = self.z_heights[point.1][point.0];

            for nearby in self.get_nearby(point.0, point.1) {
                let nearby_height = self.z_heights[nearby.1][nearby.0];
                if nearby_height < 9 && nearby_height >= height {
                    unprocessed_points.push(nearby);
                }
            }

            processed_points.insert(point);
        }

        processed_points.len()
    }

    fn get_low_points(&self) -> Vec<(usize, usize)> {
        let nearby_heights = self.get_nearby_heights();
        nearby_heights
            .into_iter()
            .filter_map(|(center_point, nearby)| {
                let center = self.z_heights[center_point.1][center_point.0];
                if nearby.iter().all(|neighbor| {
                    let neighbor = self.z_heights[neighbor.1][neighbor.0];
                    neighbor > center
                }) {
                    Some(center_point)
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

    let mut basin_sizes = low_points
        .into_iter()
        .map(|(x, y)| grid.get_basin_size(x, y))
        .collect::<Vec<_>>();

    basin_sizes.sort();
    basin_sizes.reverse();

    let total: usize = basin_sizes.iter().take(3).product();

    println!("{:?}", total);
}
