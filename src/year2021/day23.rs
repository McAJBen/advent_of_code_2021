use crate::utils::Point;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn from_char(c: char) -> Option<Amphipod> {
        match c {
            'A' => Some(Amphipod::Amber),
            'B' => Some(Amphipod::Bronze),
            'C' => Some(Amphipod::Copper),
            'D' => Some(Amphipod::Desert),
            _ => None,
        }
    }

    fn energy_cost(&self) -> u16 {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CellInput {
    is_movable: bool,
    can_stay: bool,
    room: Option<Amphipod>,
    current: Option<Amphipod>,
    is_end: bool,
}

#[derive(Debug)]
enum Cell {
    Hallway,
    Room { amphipod: Amphipod },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum MoveNum {
    Zero,
    One,
    Two,
}

impl MoveNum {
    fn increment(&self) -> Self {
        match self {
            MoveNum::Zero => MoveNum::One,
            MoveNum::One => MoveNum::Two,
            MoveNum::Two => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct AmphipodState {
    position: u8,
    move_num: MoveNum,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BurrowState {
    energy_cost: u16,
    amphipods: Vec<AmphipodState>,
}

#[derive(Debug)]
struct Burrow {
    cells: Vec<Cell>,
    amphipods: Vec<Amphipod>,
    /// (new position, cost)
    adjacency_list: Vec<Vec<(u8, u16)>>,
}

impl Burrow {
    fn new(input: &str) -> (Self, BurrowState) {
        let mut cells: Vec<Vec<CellInput>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' | ' ' => CellInput {
                            is_movable: false,
                            can_stay: false,
                            room: None,
                            current: None,
                            is_end: false,
                        },
                        '.' => CellInput {
                            is_movable: true,
                            can_stay: true,
                            room: None,
                            current: None,
                            is_end: false,
                        },
                        c => CellInput {
                            is_movable: true,
                            can_stay: true,
                            room: None,
                            current: Amphipod::from_char(c),
                            is_end: false,
                        },
                    })
                    .collect()
            })
            .collect();

        let max_depth = cells.len() - 2;

        for c in cells[max_depth].iter_mut() {
            if c.is_movable {
                c.is_end = true;
            }
        }

        let room_columns: Vec<usize> = cells[max_depth]
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if c.is_movable { Some(i) } else { None })
            .collect();

        for (index, column) in room_columns.into_iter().enumerate() {
            let room_type = match index {
                0 => Amphipod::Amber,
                1 => Amphipod::Bronze,
                2 => Amphipod::Copper,
                3 => Amphipod::Desert,
                _ => panic!("Invalid room type"),
            };
            cells[1][column].can_stay = false;
            for row in cells.iter_mut().skip(2) {
                if row[column].is_movable {
                    row[column].room = Some(room_type);
                }
            }
        }

        let mut adjacency_list = HashMap::new();

        for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.is_movable {
                    let p = Point { x, y };
                    let mut neighbors = Vec::new();

                    if p.x > 0 && cells[p.y][p.x - 1].is_movable {
                        neighbors.push(Point { x: p.x - 1, y: p.y });
                    }
                    if p.x < cells[p.y].len() - 1 && cells[p.y][p.x + 1].is_movable {
                        neighbors.push(Point { x: p.x + 1, y: p.y });
                    }
                    if p.y > 0 && cells[p.y - 1][p.x].is_movable {
                        neighbors.push(Point { x: p.x, y: p.y - 1 });
                    }
                    if p.y < cells.len() - 1 && cells[p.y + 1][p.x].is_movable {
                        neighbors.push(Point { x: p.x, y: p.y + 1 });
                    }

                    if cell.can_stay {
                        for neighbor in neighbors {
                            let e1 = adjacency_list.entry(p).or_insert_with(HashSet::new);
                            e1.insert((neighbor, 1));
                            let e2 = adjacency_list.entry(neighbor).or_insert_with(HashSet::new);
                            e2.insert((p, 1));
                        }
                    }
                }
            }
        }

        for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.is_movable && !cell.can_stay {
                    let p = Point { x, y };
                    let neighbors = adjacency_list.remove(&p).unwrap();
                    for n1 in neighbors.iter() {
                        adjacency_list.get_mut(&n1.0).unwrap().remove(&(p, 1));
                        for n2 in neighbors.iter() {
                            if n1 != n2 {
                                let e1 = adjacency_list.entry(n1.0).or_insert_with(HashSet::new);
                                e1.insert((n2.0, 2));
                                let e2 = adjacency_list.entry(n2.0).or_insert_with(HashSet::new);
                                e2.insert((n1.0, 2));
                            }
                        }
                    }
                }
            }
        }

        let point_indexes: Vec<Point> = adjacency_list.keys().cloned().collect();

        let point_map: HashMap<Point, u8> = point_indexes
            .iter()
            .enumerate()
            .map(|(index, point)| (*point, index as u8))
            .collect();

        let adjacency_list: Vec<Vec<(u8, u16)>> = point_indexes
            .iter()
            .map(|point| {
                adjacency_list
                    .get(point)
                    .unwrap()
                    .iter()
                    .map(|n| (*point_map.get(&n.0).unwrap(), n.1))
                    .collect()
            })
            .collect();

        let amphipods: Vec<(Amphipod, u8)> = point_map
            .keys()
            .filter_map(|&point| {
                cells[point.y][point.x]
                    .current
                    .map(|amphipod| (amphipod, *point_map.get(&point).unwrap()))
            })
            .collect();

        let initial_burrow_state = BurrowState {
            amphipods: amphipods
                .iter()
                .map(|(_, position)| AmphipodState {
                    position: *position,
                    move_num: MoveNum::Zero,
                })
                .collect(),
            energy_cost: 0,
        };

        let cells: Vec<Cell> = point_indexes
            .iter()
            .map(|&p| {
                let cell = cells[p.y][p.x];
                if let Some(amphipod) = cell.room {
                    Cell::Room { amphipod }
                } else {
                    Cell::Hallway
                }
            })
            .collect();

        (
            Self {
                cells,
                amphipods: amphipods.iter().map(|(amphipod, _)| *amphipod).collect(),
                adjacency_list,
            },
            initial_burrow_state,
        )
    }

    fn get_valid_moves(&self, state: &BurrowState, amphipod_index: usize) -> Vec<Move> {
        if state.amphipods[amphipod_index].move_num == MoveNum::Two {
            return Vec::new();
        }
        let base_energy_cost = self.amphipods[amphipod_index].energy_cost();
        let current_position = state.amphipods[amphipod_index].position;

        let mut moves_to_test: Vec<Move> = self.adjacency_list[current_position as usize]
            .iter()
            .map(|&(position, cost)| Move {
                energy_cost: base_energy_cost * cost,
                position,
            })
            .collect();

        let mut visited: HashSet<u8> = state.amphipods.iter().map(|a| a.position).collect();

        let mut valid_moves = Vec::new();

        while let Some(movement) = moves_to_test.pop() {
            if !visited.insert(movement.position) {
                continue;
            }

            match self.cells[movement.position as usize] {
                Cell::Hallway => {
                    if let Cell::Room { .. } = self.cells[current_position as usize] {
                        valid_moves.push(movement.clone());
                    }
                }
                Cell::Room { amphipod, .. } => {
                    if amphipod == self.amphipods[amphipod_index] {
                        valid_moves.push(movement.clone());
                    }
                }
            }

            moves_to_test.extend(self.adjacency_list[movement.position as usize].iter().map(
                |&(position, cost)| Move {
                    energy_cost: movement.energy_cost + base_energy_cost * cost,
                    position,
                },
            ));
        }

        valid_moves
    }

    fn get_moves(&self, state: &BurrowState) -> Vec<BurrowState> {
        (0..state.amphipods.len())
            .flat_map(|index| {
                // find empty space to move to
                self.get_valid_moves(state, index)
                    .into_iter()
                    .map(move |m| {
                        let mut copy = state.clone();
                        copy.amphipods[index].position = m.position;
                        copy.amphipods[index].move_num = copy.amphipods[index].move_num.increment();
                        copy.energy_cost += m.energy_cost;
                        copy
                    })
            })
            .collect()
    }

    fn is_complete(&self, state: &BurrowState) -> bool {
        state.amphipods.iter().all(|s| s.move_num == MoveNum::Two)
    }
}

#[derive(Debug, Clone)]
struct Move {
    energy_cost: u16,
    position: u8,
}

fn find_shortest(burrow: &Burrow, state: &BurrowState) -> Option<BurrowState> {
    let mut states = BinaryHeap::new();
    states.push(Reverse(state.clone()));
    let mut visited_states = HashSet::<(Vec<u8>, u16)>::new();
    visited_states.insert((
        state.amphipods.iter().map(|a| a.position).collect(),
        state.energy_cost,
    ));

    let mut i = 0;

    while let Some(Reverse(test_state)) = states.pop() {
        if burrow.is_complete(&test_state) {
            return Some(test_state);
        } else {
            let mut new_states = burrow.get_moves(&test_state);

            new_states.sort_unstable_by_key(|s| s.energy_cost);

            states.extend(
                new_states
                    .into_iter()
                    .filter(|s| {
                        visited_states.insert((
                            s.amphipods.iter().map(|a| a.position).collect(),
                            s.energy_cost,
                        ))
                    })
                    .map(Reverse),
            );
        }
        i += 1;
        if i % 10000 == 0 {
            println!("{}:{}", test_state.energy_cost, i)
        }
    }

    None
}

pub fn part1(input: &str) -> u16 {
    let (burrow, state) = Burrow::new(input);

    let state = find_shortest(&burrow, &state).unwrap();

    state.energy_cost
}

// TODO fix day 23 part 1, add part 2, and make tests
