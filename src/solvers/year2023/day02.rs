use crate::solvers::{Solver, SolverTrait};
use std::array::from_fn;

impl SolverTrait for Solver<2023, 2, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        input
            .lines()
            .filter_map(|line| {
                let line = line.strip_prefix("Game ").unwrap();
                let (game_id_str, line) = line.split_once(": ").unwrap();

                let minimums: [u32; 3] = line
                    .split("; ")
                    .map(|round_str| {
                        let mut minimums = [0; 3];

                        round_str.split(", ").for_each(|cube_str| {
                            let (num_str, color_str) = cube_str.split_once(' ').unwrap();
                            let num = u32::from_str_radix(num_str, 10).unwrap();
                            let index = match color_str {
                                "red" => 0,
                                "green" => 1,
                                "blue" => 2,
                                _ => panic!(),
                            };

                            minimums[index] = minimums[index].max(num);
                        });

                        minimums
                    })
                    .reduce(|a, b| from_fn(|i| a[i].max(b[i])))
                    .unwrap();

                if minimums[0] > 12 || minimums[1] > 13 || minimums[2] > 14 {
                    None
                } else {
                    let game_id = u32::from_str_radix(game_id_str, 10).unwrap();
                    Some(game_id)
                }
            })
            .sum::<u32>()
    }
}

impl SolverTrait for Solver<2023, 2, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        input
            .lines()
            .map(|line| {
                let (_, line) = line.split_once(": ").unwrap();
                line.split("; ")
                    .map(|round_str| {
                        let mut minimums = [0; 3];

                        round_str.split(", ").for_each(|cube_str| {
                            let (num_str, color_str) = cube_str.split_once(' ').unwrap();
                            let num = u32::from_str_radix(num_str, 10).unwrap();
                            let index = match color_str {
                                "red" => 0,
                                "green" => 1,
                                "blue" => 2,
                                _ => panic!(),
                            };

                            minimums[index] = minimums[index].max(num);
                        });

                        minimums
                    })
                    .reduce(|a, b| from_fn(|i| a[i].max(b[i])))
                    .unwrap()
                    .into_iter()
                    .reduce(|a, b| a * b)
                    .unwrap()
            })
            .sum::<u32>()
    }
}
