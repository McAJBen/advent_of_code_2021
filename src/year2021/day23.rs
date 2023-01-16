use crate::utils::Point;
use std::collections::{HashMap, HashSet};

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

    fn energy_cost(&self) -> u32 {
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
    Hallway { can_stay: bool },
    Room { amphipod: Amphipod, is_end: bool },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct AmphipodState {
    amphipod: Amphipod,
    position: Point,
    final_position: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BurrowState {
    amphipods: Vec<AmphipodState>,
    energy_cost: u32,
}

#[derive(Debug)]
struct Burrow {
    cells: HashMap<Point, Cell>,
    adjacency_list: HashMap<Point, Vec<Point>>,
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

                    for neighbor in neighbors {
                        let e1 = adjacency_list.entry(p).or_insert_with(HashSet::new);
                        e1.insert(neighbor);
                        let e2 = adjacency_list.entry(neighbor).or_insert_with(HashSet::new);
                        e2.insert(p);
                    }
                }
            }
        }

        let important_cells: Vec<Point> = adjacency_list.keys().cloned().collect();

        let burrow_state = BurrowState {
            amphipods: important_cells
                .iter()
                .filter_map(|&point| {
                    cells[point.y][point.x]
                        .current
                        .map(|amphipod| AmphipodState {
                            amphipod,
                            position: point,
                            final_position: false,
                        })
                })
                .collect(),
            energy_cost: 0,
        };

        let cells: HashMap<Point, Cell> = important_cells
            .iter()
            .map(|&p| {
                let cell = cells[p.y][p.x];
                if let Some(amphipod) = cell.room {
                    (
                        p,
                        Cell::Room {
                            amphipod,
                            is_end: cell.is_end,
                        },
                    )
                } else {
                    (
                        p,
                        Cell::Hallway {
                            can_stay: cell.can_stay,
                        },
                    )
                }
            })
            .collect();

        let adjacency_list: HashMap<Point, Vec<Point>> = important_cells
            .iter()
            .map(|&p| (p, adjacency_list[&p].iter().copied().collect::<Vec<_>>()))
            .collect();

        (
            Self {
                cells,
                adjacency_list,
            },
            burrow_state,
        )
    }

    fn get_valid_moves(&self, state: &BurrowState, amphipod_index: usize) -> Vec<Move> {
        if state.amphipods[amphipod_index].final_position {
            return Vec::new();
        }
        let base_energy_cost = state.amphipods[amphipod_index].amphipod.energy_cost();
        let current_position = state.amphipods[amphipod_index].position;

        let mut moves_to_test = self.adjacency_list[&current_position]
            .iter()
            .map(|&position| Move {
                energy_cost: base_energy_cost,
                position,
            })
            .collect::<Vec<_>>();

        let mut visited = state
            .amphipods
            .iter()
            .map(|a| a.position)
            .chain([current_position])
            .collect::<HashSet<_>>();

        let mut valid_moves = Vec::new();

        while let Some(movement) = moves_to_test.pop() {
            if visited.contains(&movement.position) {
                continue;
            }

            visited.insert(movement.position);

            match self.cells[&movement.position] {
                Cell::Hallway { can_stay } => {
                    if can_stay {
                        if let Cell::Room { .. } = self.cells[&current_position] {
                            valid_moves.push(movement.clone());
                        }
                    }
                }
                Cell::Room { amphipod, .. } => {
                    if amphipod == state.amphipods[amphipod_index].amphipod {
                        valid_moves.push(movement.clone());
                    }
                }
            }

            moves_to_test.extend(
                self.adjacency_list[&movement.position]
                    .iter()
                    .map(|&position| Move {
                        energy_cost: movement.energy_cost + base_energy_cost,
                        position,
                    }),
            );
        }

        valid_moves
    }

    fn get_moves(&self, state: &BurrowState) -> Vec<BurrowState> {
        let mut moves = Vec::new();

        for index in 0..state.amphipods.len() {
            // find empty space to move to
            let valid_moves = self
                .get_valid_moves(state, index)
                .into_iter()
                .map(|m| {
                    let mut copy = state.clone();
                    copy.amphipods[index].position = m.position;
                    copy.energy_cost += m.energy_cost;
                    self.final_position(&mut copy);
                    copy
                })
                .collect::<Vec<_>>();

            moves.extend(valid_moves);
        }

        moves
    }

    fn final_position(&self, state: &mut BurrowState) {
        let mut ends = self
            .cells
            .iter()
            .filter(|(_, c)| match c {
                Cell::Room { is_end, .. } => *is_end,
                _ => false,
            })
            .map(|(p, _)| *p)
            .collect::<Vec<_>>();

        let mut checked = HashSet::new();

        while let Some(point) = ends.pop() {
            if checked.contains(&point) {
                continue;
            }
            checked.insert(point);
            if let Cell::Room { amphipod, .. } = self.cells[&point] {
                if let Some(a) = state.amphipods.iter_mut().find(|a| a.position == point) {
                    if a.amphipod == amphipod {
                        a.final_position = true;

                        ends.extend(self.adjacency_list[&point].iter());
                    }
                }
            }
        }
    }

    fn is_complete(&self, state: &BurrowState) -> bool {
        state
            .amphipods
            .iter()
            .all(|a| match self.cells[&a.position] {
                Cell::Hallway { .. } => false,
                Cell::Room { amphipod, .. } => a.amphipod == amphipod,
            })
    }
}

#[derive(Debug, Clone)]
struct Move {
    energy_cost: u32,
    position: Point,
}

fn find_shortest(burrow: &Burrow, state: &BurrowState) -> Option<BurrowState> {
    let mut states = vec![state.clone()];
    let mut i = 0;

    while let Some(test_state) = states.pop() {
        if burrow.is_complete(&test_state) {
            return Some(test_state);
        } else {
            states.extend(burrow.get_moves(&test_state));
        }

        states.sort_by_key(|s| u32::MAX - s.energy_cost);
        i += 1;
        if i % 1000 == 0 {
            println!("{}:{}:{}", test_state.energy_cost, states.len(), i);
            states.dedup();
        }
    }

    None
}

pub fn part1(input: &str) -> u32 {
    let (burrow, state) = Burrow::new(input);

    println!("{:#?}", burrow);

    let state = find_shortest(&burrow, &state).unwrap();

    println!("{:#?}", state);

    let energy_cost = state.energy_cost;

    assert_eq!(113229032, energy_cost);

    println!("{}", energy_cost);

    energy_cost
}

// TODO fix day 23 part 1, add part 2, and make tests
