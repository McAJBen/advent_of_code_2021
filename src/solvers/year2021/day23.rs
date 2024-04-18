use crate::{
    solvers::{Solver, SolverTrait},
    utils::Point,
};
use enum_iterator::Sequence;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Hallway,
    Room { amphipod: Amphipod },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Sequence)]
enum MoveNum {
    Zero,
    One,
    Two,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AmphipodState {
    position: u8,
    move_num: MoveNum,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BurrowState {
    energy_cost: u16,
    amphipods: Vec<AmphipodState>,
}

impl PartialOrd for BurrowState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BurrowState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.energy_cost.cmp(&self.energy_cost)
    }
}

#[derive(Debug)]
struct Burrow {
    cells: Vec<Cell>,
    amphipods: Vec<Amphipod>,
    movement_paths: Vec<Vec<MovementPath>>,
}

impl Burrow {
    fn new<'a, I: IntoIterator<Item = &'a str>>(lines: I) -> (Self, BurrowState) {
        let mut cells: Vec<Vec<CellInput>> = lines
            .into_iter()
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

        let adjacency_list: Vec<HashMap<u8, u8>> = point_indexes
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

        let movement_paths = (0..cells.len())
            .map(|i| MovementPath::from_adjacency_list(&adjacency_list, i as u8))
            .collect();

        (
            Self {
                cells,
                amphipods: amphipods.iter().map(|(amphipod, _)| *amphipod).collect(),
                movement_paths,
            },
            initial_burrow_state,
        )
    }

    fn get_moves(&self, state: &BurrowState) -> Vec<BurrowState> {
        let blocked_paths: HashSet<u8> = state.amphipods.iter().map(|a| a.position).collect();

        (0..state.amphipods.len())
            .flat_map(|amphipod_index| {
                let move_num = state.amphipods[amphipod_index].move_num;
                // find empty space to move to
                if move_num == MoveNum::Two {
                    Vec::new()
                } else {
                    let current_position = state.amphipods[amphipod_index].position;

                    self.movement_paths[current_position as usize]
                        .iter()
                        .flat_map(|c| {
                            let mut v = Vec::new();
                            // TODO this more than allocating vecs everywhere
                            c.find_paths(&mut v, &blocked_paths);
                            v
                        })
                        .filter(|movement| match self.cells[movement.position as usize] {
                            Cell::Hallway => move_num == MoveNum::Zero,
                            Cell::Room { amphipod } => {
                                amphipod == self.amphipods[amphipod_index]
                                    && move_num == MoveNum::One
                            }
                        })
                        .map(move |m| {
                            let mut copy = state.clone();
                            copy.amphipods[amphipod_index].position = m.position;
                            copy.amphipods[amphipod_index].move_num =
                                enum_iterator::next(&copy.amphipods[amphipod_index].move_num)
                                    .unwrap();
                            let base_energy_cost = self.amphipods[amphipod_index].energy_cost();
                            copy.energy_cost += m.num_steps as u16 * base_energy_cost;
                            copy
                        })
                        .collect()
                }
            })
            .collect()
    }

    fn is_complete(&self, state: &BurrowState) -> bool {
        state.amphipods.iter().all(|s| s.move_num == MoveNum::Two)
    }
}

#[derive(Debug, Clone)]
struct Move {
    num_steps: u8,
    position: u8,
}

#[derive(Debug, Clone)]
struct MovementPath {
    num_steps: u8,
    position: u8,
    children: Vec<MovementPath>,
}

impl MovementPath {
    fn from_adjacency_list(
        adjacency_list: &[HashMap<u8, u8>],
        current_position: u8,
    ) -> Vec<MovementPath> {
        let mut moves_to_test: VecDeque<(u8, u8, u8)> = adjacency_list[current_position as usize]
            .iter()
            .map(|(new_position, num_steps)| (current_position, *new_position, *num_steps))
            .collect();

        let mut visited: HashSet<u8> = HashSet::new();
        visited.insert(current_position);

        let mut paths = Vec::new();

        while let Some((prev_position, new_position, steps)) = moves_to_test.pop_front() {
            if !visited.insert(new_position) {
                continue;
            }

            moves_to_test.extend(adjacency_list[new_position as usize].iter().map(
                |(new_adj_position, num_steps)| {
                    (new_position, *new_adj_position, *num_steps + steps)
                },
            ));

            if prev_position == current_position {
                paths.push(Self {
                    num_steps: steps,
                    position: new_position,
                    children: Vec::new(),
                })
            } else {
                for p in paths.iter_mut() {
                    p.add_child(prev_position, new_position, steps);
                }
            }
        }

        paths
    }

    fn add_child(&mut self, prev_position: u8, new_position: u8, steps: u8) {
        if self.position == prev_position {
            self.children.push(Self {
                num_steps: steps,
                position: new_position,
                children: Vec::new(),
            });
        } else {
            for c in self.children.iter_mut() {
                c.add_child(prev_position, new_position, steps)
            }
        }
    }

    fn find_paths(&self, vec: &mut Vec<Move>, blocked_paths: &HashSet<u8>) {
        if !blocked_paths.contains(&self.position) {
            vec.push(Move {
                num_steps: self.num_steps,
                position: self.position,
            });
            for c in self.children.iter() {
                c.find_paths(vec, blocked_paths)
            }
        }
    }
}

fn find_shortest(burrow: &Burrow, state: BurrowState) -> Option<BurrowState> {
    let mut visited_states = HashSet::<Vec<u8>>::new();

    let mut states = BinaryHeap::new();
    states.push(state);

    while let Some(test_state) = states.pop() {
        if !visited_states.insert(test_state.amphipods.iter().map(|a| a.position).collect()) {
            // already visited
            continue;
        }

        if burrow.is_complete(&test_state) {
            return Some(test_state);
        } else {
            states.extend(burrow.get_moves(&test_state));
        }
    }

    None
}

impl SolverTrait for Solver<2021, 23, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        let (burrow, state) = Burrow::new(input.lines());

        let state = find_shortest(&burrow, state).unwrap();

        state.energy_cost
    }
}

impl SolverTrait for Solver<2021, 23, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        let mut lines: Vec<&str> = input.lines().collect();
        lines.insert(3, "  #D#C#B#A#  ");
        lines.insert(4, "  #D#B#A#C#  ");
        let (burrow, state) = Burrow::new(lines);

        let state = find_shortest(&burrow, state).unwrap();

        state.energy_cost
    }
}

// TODO fix day 23 part 1, add part 2, and make tests
