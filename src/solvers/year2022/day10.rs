use crate::solvers::{Solver, SolverTrait};

impl SolverTrait for Solver<2022, 10, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        let mut register: i64 = 1;
        let mut cycle = 1;

        let mut signal_strength = 0;

        for line in input.lines() {
            let segments: Vec<&str> = line.split_whitespace().collect();
            match segments.as_slice() {
                ["noop", ..] => {
                    if (cycle + 20) % 40 == 0 {
                        signal_strength += cycle * register;
                    }
                    cycle += 1;
                }
                ["addx", value] => {
                    if (cycle + 20) % 40 == 0 {
                        signal_strength += cycle * register;
                    }
                    cycle += 1;
                    if (cycle + 20) % 40 == 0 {
                        signal_strength += cycle * register;
                    }
                    cycle += 1;
                    register += value.parse::<i64>().unwrap();
                }
                _ => panic!(),
            }
        }

        signal_strength
    }
}

impl SolverTrait for Solver<2022, 10, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        let mut register: i64 = 1;
        let mut cycle = 1;

        let mut pixels = vec![false; 240];

        for line in input.lines() {
            let segments: Vec<&str> = line.split_whitespace().collect();
            match segments.as_slice() {
                ["noop", ..] => {
                    let horizontal_pixel = (cycle - 1) % 40;
                    let sprite = (register - 1)..=(register + 1);
                    if sprite.contains(&horizontal_pixel) {
                        pixels[cycle as usize - 1] = true;
                    }
                    cycle += 1;
                }
                ["addx", value] => {
                    let horizontal_pixel = (cycle - 1) % 40;
                    let sprite = (register - 1)..=(register + 1);
                    if sprite.contains(&horizontal_pixel) {
                        pixels[cycle as usize - 1] = true;
                    }
                    cycle += 1;
                    let horizontal_pixel = (cycle - 1) % 40;
                    let sprite = (register - 1)..=(register + 1);
                    if sprite.contains(&horizontal_pixel) {
                        pixels[cycle as usize - 1] = true;
                    }
                    cycle += 1;
                    register += value.parse::<i64>().unwrap();
                }
                _ => panic!(),
            }
        }

        pixels
            .chunks(40)
            .map(|pixels| {
                pixels
                    .iter()
                    .map(|pixel| if *pixel { '#' } else { '.' })
                    .chain(['\n'])
                    .collect::<String>()
            })
            .collect::<String>()
    }
}
