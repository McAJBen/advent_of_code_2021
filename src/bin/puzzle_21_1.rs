use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct PlayerState {
    position: u16,
    score: u16,
}

impl PlayerState {
    fn new(position: u16) -> Self {
        Self { position, score: 0 }
    }

    fn take_turn(&mut self, roll: u16) {
        self.position = ((self.position + roll - 1) % 10) + 1;
        self.score += self.position;
    }
}

struct GameState {
    player1: PlayerState,
    player2: PlayerState,
    next_player: bool,
}

impl GameState {
    fn new(input: &str) -> Self {
        let starting_positions = input
            .lines()
            .map(|line| line.split_once(": ").unwrap().1.parse::<u16>().unwrap())
            .collect::<Vec<_>>();

        Self {
            player1: PlayerState::new(starting_positions[0]),
            player2: PlayerState::new(starting_positions[1]),
            next_player: true,
        }
    }

    fn take_turn(&self, roll: u16, winning_score: u16) -> (GameState, Option<bool>) {
        let mut player1 = self.player1.clone();
        let mut player2 = self.player2.clone();
        let mut winner = None;

        if self.next_player {
            player1.take_turn(roll);
            if player1.score >= winning_score {
                winner = Some(true);
            }
        } else {
            player2.take_turn(roll);
            if player2.score >= winning_score {
                winner = Some(false);
            }
        }

        (
            GameState {
                player1,
                player2,
                next_player: !self.next_player,
            },
            winner,
        )
    }
}

struct DeterministicDie {
    count: u16,
}

impl DeterministicDie {
    fn new() -> DeterministicDie {
        DeterministicDie { count: 0 }
    }

    fn roll(&mut self) -> u16 {
        self.count += 3;
        ((self.count - 1) % 100) + ((self.count - 2) % 100) + ((self.count - 3) % 100) + 3
    }

    fn num_rolls(&self) -> u16 {
        self.count
    }
}

fn main() {
    let input = read_to_string("input/21").unwrap();

    let mut game_state = GameState::new(&input);

    let mut die = DeterministicDie::new();

    let looser_score = loop {
        let (new_game_state, winner) = game_state.take_turn(die.roll(), 1000);
        if let Some(winner) = winner {
            if winner {
                break new_game_state.player2.score;
            } else {
                break new_game_state.player1.score;
            };
        } else {
            game_state = new_game_state;
        }
    };

    let value = die.num_rolls() as u32 * looser_score as u32;

    assert_eq!(916083, value);

    println!("{}", value);
}
