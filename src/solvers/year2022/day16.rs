use crate::solvers::{Solver, SolverTrait};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    rate: u8,
    next_valves: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    fn from_line(line: &'a str) -> Self {
        let line = line.strip_prefix("Valve ").unwrap();
        let (valve_name, line) = line.split_once(' ').unwrap();
        let line = line.strip_prefix("has flow rate=").unwrap();
        let (rate, line) = line.split_once(';').unwrap();
        let line = if line.starts_with(" tunnels lead to valves ") {
            line.strip_prefix(" tunnels lead to valves ").unwrap()
        } else {
            line.strip_prefix(" tunnel leads to valve ").unwrap()
        };
        let next_valves: Vec<&str> = line.split(", ").collect();
        Self {
            name: valve_name,
            rate: rate.parse().unwrap(),
            next_valves,
        }
    }
}

#[derive(Debug, Clone)]
struct ValvePathState<const MAX_MOVES: u8, const NUM_PLAYERS: usize> {
    num_moves: [u8; NUM_PLAYERS],
    total_pressure: u16,
    current_valve_index: [usize; NUM_PLAYERS],
    valves_opened: u64,
}

impl<const MAX_MOVES: u8, const NUM_PLAYERS: usize> ValvePathState<MAX_MOVES, NUM_PLAYERS> {
    fn new(starting_valve_index: [usize; NUM_PLAYERS]) -> Self {
        Self {
            num_moves: [0; NUM_PLAYERS],
            total_pressure: 0,
            current_valve_index: starting_valve_index,
            valves_opened: 0,
        }
    }

    fn add_move<'a>(
        &self,
        valve: &'a Valve<'a>,
        valve_index: usize,
        path_lengths: &[Vec<u8>],
    ) -> Option<Self> {
        if (self.valves_opened & (1 << valve_index)) != 0 {
            return None;
        }

        let (player_num, distance) = (0..NUM_PLAYERS)
            .map(|player_num| {
                let current_player_valve = self.current_valve_index[player_num];
                let distance_to_valve = path_lengths[current_player_valve][valve_index];
                (player_num, distance_to_valve)
            })
            .find(|(player_num, distance_to_valve)| {
                self.num_moves[*player_num] + distance_to_valve < MAX_MOVES
            })?;

        let mut clone = self.clone();
        clone.num_moves[player_num] += distance + 1;
        clone.total_pressure +=
            valve.rate as u16 * (MAX_MOVES - clone.num_moves[player_num]) as u16;
        clone.current_valve_index[player_num] = valve_index;
        clone.valves_opened |= 1 << valve_index;
        Some(clone)
    }
}

fn find_best_path<'a, const MAX_MOVES: u8, const NUM_PLAYERS: usize>(
    valves: &'a [Valve<'a>],
    start: ValvePathState<MAX_MOVES, NUM_PLAYERS>,
) -> ValvePathState<MAX_MOVES, NUM_PLAYERS> {
    // map valve's name back to its index
    let name_map: HashMap<&str, usize> = valves
        .iter()
        .enumerate()
        .map(|(index, v)| (v.name, index))
        .collect();

    let valve_paths: Vec<Vec<usize>> = valves
        .iter()
        .map(|valve| {
            valve
                .next_valves
                .iter()
                .map(|v| *name_map.get(v).unwrap())
                .collect()
        })
        .collect();

    let path_lengths: Vec<Vec<u8>> = (0..valves.len())
        .map(|i| {
            let mut shortest_paths = vec![u8::MAX; valves.len()];

            let mut to_test = BinaryHeap::new();
            to_test.push(Reverse((0, i)));

            while let Some(Reverse((distance, valve_index))) = to_test.pop() {
                if shortest_paths[valve_index] <= distance {
                    continue;
                }
                shortest_paths[valve_index] = distance;
                to_test.extend(
                    valve_paths[valve_index]
                        .iter()
                        .map(|v2_index| Reverse((distance + 1, *v2_index))),
                );
            }

            shortest_paths
        })
        .collect();

    let good_valves: Vec<(usize, &Valve)> = valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.rate > 0)
        .collect();

    let mut best = start;

    let mut to_test = vec![best.clone()];

    while let Some(test_path) = to_test.pop() {
        if test_path.total_pressure > best.total_pressure {
            best = test_path.clone();
        }

        for (valve_index, new_valve) in good_valves.iter() {
            if let Some(n) = test_path.add_move(new_valve, *valve_index, &path_lengths) {
                to_test.push(n);
            }
        }
    }

    best
}

impl SolverTrait for Solver<2022, 16, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        let valves: Vec<Valve> = input.lines().map(Valve::from_line).collect();

        debug_assert!(valves.len() <= 64);

        let start_valve = valves.iter().position(|v| v.name == "AA").unwrap();

        let start_state = ValvePathState::<30, 1>::new([start_valve]);

        let best = find_best_path(&valves, start_state);

        best.total_pressure
    }
}

impl SolverTrait for Solver<2022, 16, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        let valves: Vec<Valve> = input.lines().map(Valve::from_line).collect();

        debug_assert!(valves.len() <= 64);

        let start_valve = valves.iter().position(|v| v.name == "AA").unwrap();

        let start_state = ValvePathState::<26, 2>::new([start_valve, start_valve]);

        let best = find_best_path(&valves, start_state);

        best.total_pressure
    }
}
