use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use advent_of_code_2021::Point;

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
}

#[derive(Debug)]
enum Cell {
    Hallway { can_stay: bool },
    Room { amphipod: Amphipod },
}

#[derive(Debug)]
struct Burrow {
    cells: HashMap<Point, Cell>,
    adjacency_list: HashMap<Point, Vec<Point>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BurrowState {
    amphipods: Vec<(Amphipod, Vec<Point>)>,
    energy_cost: u32,
}

impl Burrow {
    fn new(input: &str) -> (Self, BurrowState) {
        let lines = input.lines().collect::<Vec<_>>();

        let mut cells = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' | ' ' => CellInput {
                            is_movable: false,
                            can_stay: false,
                            room: None,
                            current: None,
                        },
                        '.' => CellInput {
                            is_movable: true,
                            can_stay: true,
                            room: None,
                            current: None,
                        },
                        c => CellInput {
                            is_movable: true,
                            can_stay: true,
                            room: None,
                            current: Amphipod::from_char(c),
                        },
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

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

        let room_columns = lines[2]
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != '#')
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        for (index, &column) in room_columns.iter().enumerate() {
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

        let important_cells = adjacency_list.keys().cloned().collect::<Vec<_>>();

        let burrow_state = BurrowState {
            amphipods: important_cells
                .iter()
                .filter_map(|&point| cells[point.y][point.x].current.map(|a| (a, vec![point])))
                .collect(),
            energy_cost: 0,
        };

        let cells = important_cells
            .iter()
            .map(|&p| {
                let cell = cells[p.y][p.x];
                if let Some(amphipod) = cell.room {
                    (p, Cell::Room { amphipod })
                } else {
                    (
                        p,
                        Cell::Hallway {
                            can_stay: cell.can_stay,
                        },
                    )
                }
            })
            .collect::<HashMap<_, _>>();

        let adjacency_list = important_cells
            .iter()
            .map(|&p| (p, adjacency_list[&p].iter().map(|&p| p).collect::<Vec<_>>()))
            .collect::<HashMap<_, _>>();

        (
            Self {
                cells,
                adjacency_list,
            },
            burrow_state,
        )
    }

    fn get_valid_moves(&self, state: &BurrowState, amphipod_index: usize) -> Vec<Move> {
        if state.amphipods[amphipod_index].1.len() >= 3 {
            return Vec::new();
        }

        let base_energy_cost = state.amphipods[amphipod_index].0.energy_cost();
        let current_position = *state.amphipods[amphipod_index].1.last().unwrap();

        let filled_positions = state
            .amphipods
            .iter()
            .map(|(_, l)| *l.last().unwrap())
            .collect::<Vec<_>>();

        let mut moves_to_test = self.adjacency_list[&current_position]
            .iter()
            .map(|&position| Move {
                energy_cost: base_energy_cost,
                position,
            })
            .collect::<Vec<_>>();

        let mut visited = state.amphipods[amphipod_index]
            .1
            .iter()
            .copied()
            .collect::<HashSet<_>>();

        let mut valid_moves = Vec::new();

        while let Some(movement) = moves_to_test.pop() {
            if visited.contains(&movement.position) {
                continue;
            }

            visited.insert(movement.position);

            if filled_positions.contains(&movement.position) {
                continue;
            }

            match self.cells[&movement.position] {
                Cell::Hallway { can_stay } => {
                    if can_stay {
                        if let Cell::Room { .. } = self.cells[&current_position] {
                            valid_moves.push(movement.clone());
                        }
                    }
                }
                Cell::Room { amphipod } => {
                    if amphipod == state.amphipods[amphipod_index].0 {
                        if let Cell::Hallway { .. } = self.cells[&current_position] {
                            valid_moves.push(movement.clone());
                        }
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
                .filter_map(|m| {
                    let mut copy = state.clone();
                    copy.amphipods[index].1.push(m.position);
                    copy.energy_cost += m.energy_cost;

                    if copy.amphipods[index].1.len() >= 3 && !self.is_complete(&copy) {
                        None
                    } else {
                        Some(copy)
                    }
                })
                .collect::<Vec<_>>();

            moves.extend(valid_moves);
        }

        moves
    }

    fn is_complete(&self, state: &BurrowState) -> bool {
        state.amphipods.iter().all(
            |(a, positions)| match self.cells[positions.last().unwrap()] {
                Cell::Hallway { .. } => false,
                Cell::Room { amphipod } => *a == amphipod,
            },
        )
    }
}

#[derive(Debug, Clone)]
struct Move {
    energy_cost: u32,
    position: Point,
}

fn find_shortest(burrow: &Burrow, state: &BurrowState) -> Option<BurrowState> {
    let mut states = vec![state.clone()];

    while let Some(state) = states.pop() {
        println!("{:?}", state);
        if state.energy_cost > 12521 {
            panic!("{:?}", state);
        }
        if burrow.is_complete(&state) {
            return Some(state);
        } else {
            states.extend(burrow.get_moves(&state));
        }

        states.sort_by_key(|s| u32::MAX - s.energy_cost);
    }

    None
}

fn main() {
    panic!("This doesn't work");
    let input = read_to_string("input/23").unwrap();

    let (burrow, state) = Burrow::new(&input);

    println!("{:#?}", burrow);

    let state = find_shortest(&burrow, &state).unwrap();

    println!("{:#?}", state);

    let energy_cost = state.energy_cost;

    assert_eq!(113229032, energy_cost);

    println!("{}", energy_cost);
}
