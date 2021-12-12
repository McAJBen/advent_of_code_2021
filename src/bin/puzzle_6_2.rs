use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("puzzle_6_input").unwrap();

    let fishes: Vec<u8> = input.split(',').map(|x| x.parse().unwrap()).collect();

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

    let total = map.values().sum::<u128>();

    assert_eq!(1743335992042, total);

    println!("{}", total);
}
