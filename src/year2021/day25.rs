use crate::utils::{read_input, Point};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CucumberState {
    East,
    South,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SeaFloor {
    spaces: Vec<Vec<CucumberState>>,
    width: usize,
    height: usize,
}

impl Display for SeaFloor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = self
            .spaces
            .iter()
            .flat_map(|row| {
                row.iter()
                    .map(|space| match space {
                        CucumberState::East => '>',
                        CucumberState::South => 'v',
                        CucumberState::Empty => '.',
                    })
                    .chain(['\n'])
            })
            .collect::<String>();

        write!(f, "{}", s)
    }
}

impl SeaFloor {
    fn new(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();

        let width = lines[0].len();
        let height = lines.len();

        let spaces = lines
            .into_iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => CucumberState::Empty,
                        '>' => CucumberState::East,
                        'v' => CucumberState::South,
                        _ => panic!("Invalid character: {}", c),
                    })
                    .collect()
            })
            .collect();

        SeaFloor {
            spaces,
            width,
            height,
        }
    }

    fn step(&self) -> Self {
        let mut new_floor = self.clone();

        let mut moving_cucumbers = Vec::new();

        for (y, row) in new_floor.spaces.iter().enumerate() {
            for (x, space) in row.iter().enumerate() {
                if *space == CucumberState::East {
                    let next_x = (x + 1) % new_floor.width;
                    if new_floor.spaces[y][next_x] == CucumberState::Empty {
                        moving_cucumbers.push(Point::new(x, y));
                    }
                }
            }
        }

        for Point { x, y } in moving_cucumbers {
            new_floor.spaces[y][x] = CucumberState::Empty;
            new_floor.spaces[y][(x + 1) % new_floor.width] = CucumberState::East;
        }

        let mut moving_cucumbers = Vec::new();

        for (y, row) in new_floor.spaces.iter().enumerate() {
            for (x, space) in row.iter().enumerate() {
                if *space == CucumberState::South {
                    let next_y = (y + 1) % new_floor.height;
                    if new_floor.spaces[next_y][x] == CucumberState::Empty {
                        moving_cucumbers.push(Point::new(x, y));
                    }
                }
            }
        }

        for Point { x, y } in moving_cucumbers {
            new_floor.spaces[y][x] = CucumberState::Empty;
            new_floor.spaces[(y + 1) % new_floor.height][x] = CucumberState::South;
        }

        new_floor
    }
}

pub fn part1() -> u16 {
    let input = read_input(2021, 25);

    let mut prev_floor = SeaFloor::new(&input);
    let mut iterations = 0;

    loop {
        let floor = prev_floor.step();
        iterations += 1;

        if prev_floor == floor {
            break;
        }
        prev_floor = floor;
    }

    iterations
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 557);
}
