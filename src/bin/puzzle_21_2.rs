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

fn main() {
    let input = read_to_string("input/21").unwrap();

    let mut game_states = vec![(1, GameState::new(&input))];
    let mut player1_wins: u64 = 0;
    let mut player2_wins: u64 = 0;

    while let Some((count, game_state)) = game_states.pop() {
        for (multiple, roll) in [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)] {
            let (new_game_state, winner) = game_state.take_turn(roll, 21);

            if let Some(winner) = winner {
                if winner {
                    player1_wins += count * multiple;
                } else {
                    player2_wins += count * multiple;
                }
            } else {
                game_states.push((count * multiple, new_game_state));
            }
        }
    }

    let value = player1_wins.max(player2_wins);

    assert_eq!(49982165861983, value);

    println!("{}", value);
}
