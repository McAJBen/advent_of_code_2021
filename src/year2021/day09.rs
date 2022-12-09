use crate::utils::{read_input, Point};
use std::collections::HashSet;

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

    fn get_nearby(&self, point: Point) -> HashSet<Point> {
        let Point { x, y } = point;
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
        nearby_heights
    }

    fn get_nearby_heights(&self) -> Vec<(Point, HashSet<Point>)> {
        let mut heights = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point { x, y };
                heights.push((point, self.get_nearby(point)));
            }
        }
        heights
    }

    fn get_basin_size(&self, low_point: Point) -> usize {
        let mut processed_points = HashSet::new();
        let mut unprocessed_points = vec![low_point];

        while let Some(point) = unprocessed_points.pop() {
            if processed_points.contains(&point) {
                continue;
            }

            let height = self.z_heights[point.y][point.x];

            for nearby in self.get_nearby(point) {
                let nearby_height = self.z_heights[nearby.y][nearby.x];
                if nearby_height < 9 && nearby_height >= height {
                    unprocessed_points.push(nearby);
                }
            }

            processed_points.insert(point);
        }

        processed_points.len()
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

    fn get_low_points2(&self) -> Vec<Point> {
        let nearby_heights = self.get_nearby_heights();
        nearby_heights
            .into_iter()
            .filter_map(|(center_point, nearby)| {
                let center = self.z_heights[center_point.y][center_point.x];
                if nearby.iter().all(|neighbor| {
                    let neighbor = self.z_heights[neighbor.y][neighbor.x];
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

pub fn part1(input: &str) -> u32 {
    let grid = Grid::new(input.lines().collect());

    let low_points = grid.get_low_points();

    low_points.into_iter().map(|x| x as u32 + 1).sum::<u32>()
}

#[test]
fn test_part1() {
    let input = read_input(2021, 9);
    assert_eq!(part1(&input), 594);
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::new(input.lines().collect());

    let low_points = grid.get_low_points2();

    let mut basin_sizes = low_points
        .into_iter()
        .map(|p| grid.get_basin_size(p))
        .collect::<Vec<_>>();

    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    basin_sizes.iter().take(3).product()
}

#[test]
fn test_part2() {
    let input = read_input(2021, 9);
    assert_eq!(part2(&input), 858494);
}
