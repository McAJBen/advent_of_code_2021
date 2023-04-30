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

#[derive(Debug, Default, Clone)]
struct ValvePath<'a> {
    num_moves: u8,
    total_pressure: u16,
    valves: Vec<&'a Valve<'a>>,
}

impl<'a> ValvePath<'a> {
    fn add(&mut self, distance: u8, valve: &'a Valve<'a>) {
        self.num_moves += distance + 1;
        self.total_pressure += valve.rate as u16 * (30 - self.num_moves) as u16;
        self.valves.push(valve);
    }
}

pub fn part1(input: &str) -> u16 {
    let valves: Vec<Valve> = input.lines().map(|line| Valve::from_line(line)).collect();

    let name_map: HashMap<&str, usize> = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name, i))
        .collect();

    let path_lengths: HashMap<&str, HashMap<&str, u8>> = (0..valves.len())
        .map(|i| {
            let mut paths = HashMap::new();

            let mut to_test = BinaryHeap::new();
            to_test.push(Reverse((0, i)));

            while let Some(Reverse((distance, valve_index))) = to_test.pop() {
                let valve = &valves[valve_index];
                if *paths.get(valve.name).unwrap_or(&u8::MAX) < distance {
                    continue;
                }
                paths.insert(valve.name, distance);

                to_test.extend(
                    valve
                        .next_valves
                        .iter()
                        .map(|v| Reverse((distance + 1, *name_map.get(v).unwrap()))),
                );
            }

            (valves[i].name, paths)
        })
        .collect();

    let mut best = ValvePath::default();
    let mut to_test = vec![ValvePath::default()];

    while let Some(test_path) = to_test.pop() {
        if test_path.total_pressure > best.total_pressure {
            best = test_path.clone();
        }

        let current_valve = test_path
            .valves
            .last()
            .cloned()
            .unwrap_or_else(|| valves.iter().find(|v| v.name == "AA").unwrap());

        for new_valve in valves.iter() {
            if new_valve.rate == 0 {
                continue;
            }
            if test_path.valves.iter().any(|v| v.name == new_valve.name) {
                continue;
            }
            let distance = *path_lengths
                .get(current_valve.name)
                .unwrap()
                .get(new_valve.name)
                .unwrap();

            if test_path.num_moves + distance >= 30 {
                continue;
            }

            let mut new_path = test_path.clone();
            new_path.add(distance, new_valve);

            to_test.push(new_path);
        }
    }

    println!("{:?}", best);

    best.total_pressure
}

pub fn part2(input: &str) -> u64 {
    0
}
