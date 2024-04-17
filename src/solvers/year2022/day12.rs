use crate::solvers::{Solver, SolverTrait};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn get_heights(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' | 'a' => 0,
                    'E' => 25,
                    c => c as u8 - b'a',
                })
                .collect()
        })
        .collect()
}

fn get_neighbors(heights: &[Vec<u8>]) -> Vec<Vec<usize>> {
    let height = heights.len();
    let width = heights[0].len();

    let mut neighbors = vec![Vec::new(); width * height];

    for y in 0..height {
        for x in 0..width {
            if let Some(neg_y) = y.checked_sub(1) {
                if heights[neg_y][x] <= heights[y][x] + 1 {
                    neighbors[y * width + x].push(neg_y * width + x);
                }
            }
            if let Some(neg_x) = x.checked_sub(1) {
                if heights[y][neg_x] <= heights[y][x] + 1 {
                    neighbors[y * width + x].push(y * width + neg_x);
                }
            }
            let pos_y = y + 1;
            if pos_y < height && heights[pos_y][x] <= heights[y][x] + 1 {
                neighbors[y * width + x].push(pos_y * width + x);
            }
            let pos_x = x + 1;
            if pos_x < width && heights[y][pos_x] <= heights[y][x] + 1 {
                neighbors[y * width + x].push(y * width + pos_x);
            }
        }
    }
    neighbors
}

fn dijkstra(starts: &[usize], end: usize, neighbors: &[Vec<usize>]) -> usize {
    let mut finished_nodes = HashSet::new();

    let mut pending_nodes = BinaryHeap::new();
    for start in starts {
        pending_nodes.push(Reverse((0, *start)));
    }

    while let Some(Reverse((path_length, node))) = pending_nodes.pop() {
        if end == node {
            return path_length;
        }

        if finished_nodes.insert(node) {
            for neighbor in neighbors[node].iter() {
                pending_nodes.push(Reverse((path_length + 1, *neighbor)));
            }
        }
    }
    panic!();
}

impl SolverTrait<usize> for Solver<2022, 12, 1> {
    fn solve(&self, input: &str) -> usize {
        let heights = get_heights(input);
        let neighbors = get_neighbors(&heights);

        let width = heights[0].len();

        let start = input
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .find_map(|(x, c)| if c == 'S' { Some(y * width + x) } else { None })
            })
            .unwrap();

        let ends = input
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .find_map(|(x, c)| if c == 'E' { Some(y * width + x) } else { None })
            })
            .unwrap();

        dijkstra(&[start], ends, &neighbors)
    }
}

impl SolverTrait<usize> for Solver<2022, 12, 2> {
    fn solve(&self, input: &str) -> usize {
        let heights = get_heights(input);
        let neighbors = get_neighbors(&heights);

        let width = heights[0].len();

        let starts: Vec<usize> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if c == 'a' || c == 'S' {
                            Some(y * width + x)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let ends = input
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .find_map(|(x, c)| if c == 'E' { Some(y * width + x) } else { None })
            })
            .unwrap();

        dijkstra(&starts, ends, &neighbors)
    }
}
