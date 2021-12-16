use advent_of_code::Point;
use std::fs::read_to_string;

#[derive(Debug)]
struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
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

        let mut data = vec![];

        for d_y in 0..5 {
            for row in original_data.iter() {
                for d_x in 0..5 {
                    for value in row.iter() {
                        let mod_value = (value + d_y + d_x - 1) % 9 + 1;
                        data.push(mod_value);
                    }
                }
            }
        }

        Self {
            data,
            width: original_width * 5,
            height: original_height * 5,
        }
    }

    fn walk(&self) -> u64 {
        let mut best_risks: Vec<Option<u64>> = vec![None; self.width * self.height];
        let mut updatable_points = vec![0];

        while let Some(index) = updatable_points.pop() {
            let point = Point::new(index % self.width, index / self.width);
            let mut neighbors = Vec::new();
            if point.x > 0 {
                neighbors.push(index - 1);
            }
            if point.x < self.width - 1 {
                neighbors.push(index + 1);
            }
            if point.y > 0 {
                neighbors.push(index - self.width);
            }
            if point.y < self.height - 1 {
                neighbors.push(index + self.width);
            }

            let best_risk = if index == 0 {
                0
            } else {
                let self_risk = self.data[index] as u64;

                let min_risk = neighbors
                    .iter()
                    .filter_map(|neighbor| best_risks[*neighbor])
                    .min()
                    .unwrap();
                min_risk + self_risk
            };

            if best_risks[index].map_or(true, |previous| previous > best_risk) {
                best_risks[index] = Some(best_risk);
                updatable_points.extend(neighbors);
            }
        }

        best_risks.last().unwrap().unwrap()
    }
}

fn main() {
    let input = read_to_string("puzzle_15_input").unwrap();

    let grid = Grid::new(input);

    let total_risk = grid.walk();

    assert_eq!(2963, total_risk);

    println!("{}", total_risk);
}
