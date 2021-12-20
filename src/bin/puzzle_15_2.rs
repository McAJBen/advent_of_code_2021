use advent_of_code_2021::Point;
use std::{collections::BinaryHeap, fs::read_to_string, panic};

struct ToVisit {
    position: usize,
    risk: u16,
}

impl Ord for ToVisit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for ToVisit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ToVisit {
    fn eq(&self, other: &Self) -> bool {
        self.risk.eq(&other.risk)
    }
}

impl Eq for ToVisit {}

#[derive(Debug)]
struct Grid {
    risks: Vec<u8>,
    adjacency_list: Vec<Vec<usize>>,
}

impl Grid {
    fn new(input: String) -> Self {
        let lines = input.lines().collect::<Vec<_>>();

        let original_height = lines.len();
        let original_width = lines[0].len();

        let original_data = lines
            .into_iter()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut risks = Vec::new();

        for d_y in 0..5 {
            for row in original_data.iter() {
                for d_x in 0..5 {
                    for value in row.iter() {
                        let mod_value = (value + d_y + d_x - 1) % 9 + 1;
                        risks.push(mod_value);
                    }
                }
            }
        }

        let width = original_width * 5;
        let height = original_height * 5;

        let adjacency_list = (0..risks.len())
            .map(|index| {
                let point = Point::new(index % width, index / width);
                let mut neighbors = Vec::new();
                if point.x > 0 {
                    neighbors.push(index - 1);
                }
                if point.x < width - 1 {
                    neighbors.push(index + 1);
                }
                if point.y > 0 {
                    neighbors.push(index - width);
                }
                if point.y < height - 1 {
                    neighbors.push(index + width);
                }
                neighbors
            })
            .collect::<Vec<_>>();

        Self {
            risks,
            adjacency_list,
        }
    }

    fn dijkstra(&self) -> u16 {
        let mut risks = vec![u16::MAX; self.adjacency_list.len()];
        let mut visited = vec![false; self.adjacency_list.len()];
        let mut to_visit = BinaryHeap::new();

        risks[0] = 0;
        to_visit.push(ToVisit {
            position: 0,
            risk: 0,
        });

        let end = self.adjacency_list.len() - 1;

        while let Some(current) = to_visit.pop() {
            if current.position == end {
                return current.risk;
            }
            if visited[current.position] {
                continue;
            }

            for &neighbor_index in self.adjacency_list[current.position].iter() {
                if !visited[neighbor_index] {
                    let old_risk = risks[neighbor_index];
                    let new_risk = current.risk + self.risks[neighbor_index] as u16;

                    if new_risk < old_risk {
                        risks[neighbor_index] = new_risk;
                        to_visit.push(ToVisit {
                            position: neighbor_index,
                            risk: new_risk,
                        });
                    }
                }
            }
            visited[current.position] = true;
        }

        panic!("No path found");
    }
}

fn main() {
    let input = read_to_string("input/15").unwrap();

    let grid = Grid::new(input);

    let total_risk = grid.dijkstra();

    assert_eq!(2963, total_risk);

    println!("{}", total_risk);
}
