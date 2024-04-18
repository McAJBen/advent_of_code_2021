use crate::{
    solvers::{Solver, SolverTrait},
    utils::Point,
};
use enum_iterator::Sequence;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WindJet {
    Left,
    Right,
}

impl WindJet {
    fn from_str(input: &str) -> Vec<Self> {
        input
            .chars()
            .map(|c| match c {
                '<' => Self::Left,
                '>' => Self::Right,
                _ => panic!(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Sequence)]
enum Rock {
    HorizontalLine,
    Cross,
    Corner,
    VerticalLine,
    Square,
}

impl Rock {
    fn get_coordinates(&self) -> Vec<Point> {
        match self {
            Rock::HorizontalLine => vec![
                Point::new(2, 3),
                Point::new(3, 3),
                Point::new(4, 3),
                Point::new(5, 3),
            ],
            Rock::Cross => vec![
                Point::new(3, 3),
                Point::new(2, 4),
                Point::new(3, 4),
                Point::new(4, 4),
                Point::new(3, 5),
            ],
            Rock::Corner => vec![
                Point::new(2, 3),
                Point::new(3, 3),
                Point::new(4, 3),
                Point::new(4, 4),
                Point::new(4, 5),
            ],
            Rock::VerticalLine => vec![
                Point::new(2, 3),
                Point::new(2, 4),
                Point::new(2, 5),
                Point::new(2, 6),
            ],
            Rock::Square => vec![
                Point::new(2, 3),
                Point::new(3, 3),
                Point::new(2, 4),
                Point::new(3, 4),
            ],
        }
    }
}

#[derive(Debug, Default)]
struct Chamber {
    squares: VecDeque<[bool; 7]>,
    hidden_height: usize,
}

impl Chamber {
    fn add_rock(&mut self, jet_pattern: &mut impl Iterator<Item = WindJet>, rock: Rock) {
        let mut rock_coordinates = rock.get_coordinates();

        for p in rock_coordinates.iter_mut() {
            p.y += self.squares.len();
        }

        loop {
            let jet = jet_pattern.next().unwrap();

            if self.can_move_horizontally(&rock_coordinates, jet) {
                for p in rock_coordinates.iter_mut() {
                    match jet {
                        WindJet::Left => p.x -= 1,
                        WindJet::Right => p.x += 1,
                    }
                }
            }

            if self.can_move_down(&rock_coordinates) {
                for p in rock_coordinates.iter_mut() {
                    p.y -= 1
                }
            } else {
                self.save_rock(rock_coordinates);
                break;
            }
        }
    }

    fn can_move_horizontally(&self, rock_coordinates: &[Point], wind_jet: WindJet) -> bool {
        rock_coordinates.iter().all(|p| {
            if match wind_jet {
                WindJet::Left => p.x == 0,
                WindJet::Right => p.x == 6,
            } {
                return false;
            }
            let new_x = match wind_jet {
                WindJet::Left => p.x - 1,
                WindJet::Right => p.x + 1,
            };

            match self.squares.get(p.y) {
                Some(row) => !row[new_x],
                None => true,
            }
        })
    }

    fn can_move_down(&self, rock_coordinates: &[Point]) -> bool {
        rock_coordinates.iter().all(|p| {
            if p.y == 0 {
                return false;
            }

            match self.squares.get(p.y - 1) {
                Some(row) => !row[p.x],
                None => true,
            }
        })
    }

    fn save_rock(&mut self, rock_coordinates: Vec<Point>) {
        for rock_coordinate in rock_coordinates {
            while self.squares.len() <= rock_coordinate.y {
                self.squares.push_back([false; 7]);
            }

            self.squares[rock_coordinate.y][rock_coordinate.x] = true;
        }

        if self.squares.len() > 100_000 {
            self.squares.drain(..95_000);
            self.hidden_height += 95_000;
        }
    }

    fn height(&self) -> usize {
        self.squares.len() + self.hidden_height
    }
}

impl SolverTrait for Solver<2022, 17, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        let jets = WindJet::from_str(input);
        let mut jet_pattern = jets.into_iter().cycle();
        let mut chamber = Chamber::default();

        for rock in enum_iterator::all::<Rock>().cycle().take(2022) {
            chamber.add_rock(&mut jet_pattern, rock);
        }

        chamber.height()
    }
}

impl SolverTrait for Solver<2022, 17, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        let jets = WindJet::from_str(input);
        let mut jet_pattern = jets.into_iter().cycle();
        let mut chamber = Chamber::default();

        let progress_bar = ProgressBar::new(1_000_000_000_000).with_style(
            ProgressStyle::default_bar()
                .template("[{eta_precise}] {bar:40.cyan/blue} {percent} {pos:>7}/{len:7} {msg}")
                .unwrap(),
        );

        for rock in enum_iterator::all::<Rock>()
            .cycle()
            .take(1_000_000_000_000)
            .progress_with(progress_bar)
        {
            chamber.add_rock(&mut jet_pattern, rock);
        }

        chamber.height()
    }
}
