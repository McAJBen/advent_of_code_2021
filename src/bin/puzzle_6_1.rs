use advent_of_code::read_lines;
use std::fs::File;

fn main() {
    let input = File::open("puzzle_6_input").unwrap();

    let mut fishes: Vec<u8> = read_lines(&input)[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    for _ in 0..80 {
        let mut num_new_fish = 0;

        for fish in fishes.iter_mut() {
            if *fish == 0 {
                num_new_fish += 1;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }

        fishes.extend(vec![8; num_new_fish]);
    }

    println!("{:?}", fishes.len());
}
