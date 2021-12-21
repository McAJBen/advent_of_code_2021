use std::fs::read_to_string;

trait Die {
    fn roll(&mut self) -> u16;
}

struct DeterministicDie {
    count: u16,
}

impl DeterministicDie {
    fn new() -> DeterministicDie {
        DeterministicDie { count: 0 }
    }

    fn num_rolls(&self) -> u16 {
        self.count
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> u16 {
        self.count += 1;
        ((self.count - 1) % 100) + 1
    }
}

fn main() {
    let input = read_to_string("input/21").unwrap();

    let starting_positions = input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<u16>().unwrap())
        .collect::<Vec<_>>();

    let mut player1_position = starting_positions[0];
    let mut player2_position = starting_positions[1];

    let mut player1_score: u16 = 0;
    let mut player2_score: u16 = 0;

    let mut die = DeterministicDie::new();

    let looser_score = loop {
        let roll = die.roll() + die.roll() + die.roll();
        player1_position = ((player1_position + roll - 1) % 10) + 1;
        player1_score += player1_position as u16;

        if player1_score >= 1000 {
            break player2_score;
        }

        let roll = die.roll() + die.roll() + die.roll();
        player2_position = ((player2_position + roll - 1) % 10) + 1;
        player2_score += player2_position as u16;

        if player2_score >= 1000 {
            break player1_score;
        }
    };

    let value = die.num_rolls() as u32 * looser_score as u32;

    assert_eq!(916083, value);

    println!("{}", value);
}
