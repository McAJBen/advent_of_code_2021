use advent_of_code_2021::Point;
use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    z_heights: Vec<Vec<u8>>,
}

impl Grid {
    fn new(lines: Vec<&str>) -> Self {
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

    fn get_nearby_heights(&self) -> Vec<(Point, HashSet<Point>)> {
        let mut heights = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut nearby_heights = HashSet::new();
                if x < self.width - 1 {
                    nearby_heights.insert(Point { x: x + 1, y });
                }
                if x > 0 {
                    nearby_heights.insert(Point { x: x - 1, y });
                }
                if y < self.height - 1 {
                    nearby_heights.insert(Point { x, y: y + 1 });
                }
                if y > 0 {
                    nearby_heights.insert(Point { x, y: y - 1 });
                }
                heights.push((Point { x, y }, nearby_heights));
            }
        }
        heights
    }

    fn get_low_points(&self) -> Vec<u8> {
        let nearby_heights = self.get_nearby_heights();
        nearby_heights
            .into_iter()
            .filter_map(|(center, nearby)| {
                let center = self.z_heights[center.y][center.x];
                if nearby.iter().all(|neighbor| {
                    let neighbor = self.z_heights[neighbor.y][neighbor.x];
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
    let input = read_to_string("input/9").unwrap();

    let grid = Grid::new(input.lines().collect());

    let low_points = grid.get_low_points();

    let total = low_points.into_iter().map(|x| x as u128 + 1).sum::<u128>();

    assert_eq!(594, total);

    println!("{}", total);
}
