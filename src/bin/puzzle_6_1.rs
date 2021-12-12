use std::fs::read_to_string;

fn main() {
    let input = read_to_string("puzzle_6_input").unwrap();

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

    let total = fishes.len();

    assert_eq!(389726, total);

    println!("{}", total);
}
