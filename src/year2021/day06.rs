use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let mut fishes: Vec<u8> = input.split(',').map(|x| x.parse().unwrap()).collect();

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

    fishes.len()
}

pub fn part2(input: &str) -> u64 {
    let fishes: Vec<u8> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let mut map = HashMap::new();

    for fish in fishes {
        *map.entry(fish).or_insert(0) += 1;
    }

    for _ in 0..256 {
        let mut new_map: HashMap<u8, u64> = HashMap::new();

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

    map.values().sum::<u64>()
}
