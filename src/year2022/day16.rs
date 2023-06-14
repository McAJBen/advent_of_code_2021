use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
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

#[test]
fn valve_from_line() {
    let valve = Valve::from_line("Valve PZ has flow rate=14; tunnels lead to valves KU, HE");
    assert_eq!(valve.name, "PZ");
    assert_eq!(valve.rate, 14);
    assert_eq!(valve.next_valves, vec!["KU", "HE"]);

    let valve = Valve::from_line("Valve PP has flow rate=25; tunnel leads to valve KU");
    assert_eq!(valve.name, "PP");
    assert_eq!(valve.rate, 25);
    assert_eq!(valve.next_valves, vec!["KU"]);

    let valve =
        Valve::from_line("Valve WA has flow rate=6; tunnels lead to valves TX, AF, RG, HU, NY");
    assert_eq!(valve.name, "WA");
    assert_eq!(valve.rate, 6);
    assert_eq!(valve.next_valves, vec!["TX", "AF", "RG", "HU", "NY"]);
}

#[derive(Debug, Clone)]
struct ValvePathState<'a, const MAX_MOVES: u8, const NUM_PLAYERS: usize> {
    num_moves: [u8; NUM_PLAYERS],
    total_pressure: u16,
    current_valve: [&'a Valve<'a>; NUM_PLAYERS],
    valves_opened: HashSet<&'a str>,
}

impl<'a, const MAX_MOVES: u8, const NUM_PLAYERS: usize> ValvePathState<'a, MAX_MOVES, NUM_PLAYERS> {
    fn new(starting_valves: [&'a Valve<'a>; NUM_PLAYERS]) -> Self {
        Self {
            num_moves: [0; NUM_PLAYERS],
            total_pressure: 0,
            current_valve: starting_valves,
            valves_opened: HashSet::new(),
        }
    }

    fn add_move(
        &self,
        valve: &'a Valve<'a>,
        path_lengths: &HashMap<&str, HashMap<&str, u8>>,
    ) -> Option<Self> {
        if self.valves_opened.contains(valve.name) {
            return None;
        }

        let mut player_num = 0;
        let mut distance: u8;

        loop {
            distance = *path_lengths
                .get(self.current_valve[player_num].name)
                .unwrap()
                .get(valve.name)
                .unwrap();

            if self.num_moves[player_num] + distance >= MAX_MOVES {
                if player_num == NUM_PLAYERS - 1 {
                    return None;
                } else {
                    player_num += 1;
                }
            } else {
                break;
            }
        }

        let mut clone = self.clone();
        clone.num_moves[player_num] += distance + 1;
        clone.total_pressure +=
            valve.rate as u16 * (MAX_MOVES - clone.num_moves[player_num]) as u16;
        clone.current_valve[player_num] = valve;
        clone.valves_opened.insert(valve.name);
        return Some(clone);
    }
}

fn find_best_path<'a, const MAX_MOVES: u8, const NUM_PLAYERS: usize>(
    valves: &'a [Valve<'a>],
    start: ValvePathState<'a, MAX_MOVES, NUM_PLAYERS>,
) -> ValvePathState<'a, MAX_MOVES, NUM_PLAYERS> {
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

    let path_lengths: HashMap<&str, HashMap<&str, u8>> = (0..valves.len())
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

            (
                valves[i].name,
                shortest_paths
                    .into_iter()
                    .enumerate()
                    .map(|(i, length)| (valves[i].name, length))
                    .collect(),
            )
        })
        .collect();

    let good_valves: Vec<&Valve> = valves.iter().filter(|v| v.rate > 0).collect();

    let mut best = start;

    let mut to_test = vec![best.clone()];

    while let Some(test_path) = to_test.pop() {
        if test_path.total_pressure > best.total_pressure {
            best = test_path.clone();
        }

        for new_valve in good_valves.iter() {
            if let Some(n) = test_path.add_move(new_valve, &path_lengths) {
                to_test.push(n);
            }
        }
    }

    best
}

pub fn part1(input: &str) -> u16 {
    let valves: Vec<Valve> = input.lines().map(|line| Valve::from_line(line)).collect();

    let start_valve = valves.iter().find(|v| v.name == "AA").unwrap();

    let best = find_best_path(&valves, ValvePathState::<30, 1>::new([start_valve]));

    best.total_pressure
}

pub fn part2(input: &str) -> u16 {
    let valves: Vec<Valve> = input.lines().map(|line| Valve::from_line(line)).collect();

    let start_valve = valves.iter().find(|v| v.name == "AA").unwrap();

    let best = find_best_path(
        &valves,
        ValvePathState::<26, 2>::new([start_valve, start_valve]),
    );

    best.total_pressure
}
