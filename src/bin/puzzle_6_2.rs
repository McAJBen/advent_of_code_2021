use advent_of_code::read_lines;
use std::{collections::HashMap, fs::File};

fn main() {
    let input = File::open("puzzle_6_input").unwrap();

    let fishes: Vec<u8> = read_lines(&input)[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut map = HashMap::new();

    for fish in fishes {
        *map.entry(fish).or_insert(0) += 1;
    }

    for _ in 0..256 {
        let mut new_map: HashMap<u8, u128> = HashMap::new();

        for (key, value) in map.into_iter() {
            if key == 0 {
                *new_map.entry(6).or_insert(0) += value;
                *new_map.entry(8).or_insert(0) += value;
            } else {
                *new_map.entry(key - 1).or_insert(0) += value;
            }
        }

        map = new_map;
    }

    println!("{:?}", map.values().sum::<u128>());
}
