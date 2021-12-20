use advent_of_code_2021::Point;
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

        let height = lines.len();
        let width = lines[0].len();

        let data = lines
            .into_iter()
            .flat_map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect();

        Self {
            data,
            width,
            height,
        }
    }

    fn walk(&self) -> u16 {
        let mut routes = vec![0u16];

        for index in 1..(self.width * self.height) {
            let point = Point::new(index % self.width, index / self.width);
            let risk = self.data[index] as u16;

            let mut possible_routes = vec![];

            if point.x > 0 {
                possible_routes.push(routes[index - 1] + risk);
            }

            if point.y > 0 {
                possible_routes.push(routes[index - self.width] + risk);
            }

            routes.push(possible_routes.into_iter().min().unwrap());
        }

        *routes.last().unwrap()
    }
}

fn main() {
    let input = read_to_string("input/15").unwrap();

    let grid = Grid::new(input);

    let total_risk = grid.walk();

    assert_eq!(769, total_risk);

    println!("{}", total_risk);
}
