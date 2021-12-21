use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct PlayerState {
    position: u8,
    score: u8,
}

impl PlayerState {
    fn new(position: u8) -> Self {
        Self { position, score: 0 }
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
            .map(|line| line.split_once(": ").unwrap().1.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        Self {
            player1: PlayerState::new(starting_positions[0]),
            player2: PlayerState::new(starting_positions[1]),
            next_player: true,
        }
    }

    fn take_turn(&self, roll: u8) -> GameState {
        let mut player1 = self.player1.clone();
        let mut player2 = self.player2.clone();
        if self.next_player {
            player1.position = ((player1.position + roll - 1) % 10) + 1;
            player1.score += player1.position;
        } else {
            player2.position = ((player2.position + roll - 1) % 10) + 1;
            player2.score += player2.position;
        }
        GameState {
            player1,
            player2,
            next_player: !self.next_player,
        }
    }

    fn winner(&self) -> Option<bool> {
        if self.player1.score >= 21 {
            Some(true)
        } else if self.player2.score >= 21 {
            Some(false)
        } else {
            None
        }
    }
}

fn main() {
    let input = read_to_string("input/21").unwrap();

    let mut game_states = vec![(1, GameState::new(&input))];
    let mut player1_wins: u64 = 0;
    let mut player2_wins: u64 = 0;

    while let Some((count, game_state)) = game_states.pop() {
        if let Some(winner) = game_state.winner() {
            if winner {
                player1_wins += count;
            } else {
                player2_wins += count;
            }
        } else {
            for (multiple, roll) in [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)] {
                game_states.push((count * multiple, game_state.take_turn(roll)));
            }
        }
    }

    let value = player1_wins.max(player2_wins);

    assert_eq!(49982165861983, value);

    println!("{}", value);
}
