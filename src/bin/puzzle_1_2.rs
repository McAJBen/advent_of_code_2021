use advent_of_code::{read_lines, ZipWithNext};
use std::fs::File;

fn main() {
    let input = File::open("puzzle_1_input").unwrap();

    let depths = read_lines(&input)
        .into_iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut depth_groups = depths
        .iter()
        .skip(2)
        .map(|depth| *depth)
        .collect::<Vec<_>>();

    for (index, depth) in depths.iter().skip(1).take(depth_groups.len()).enumerate() {
        depth_groups[index] += depth;
    }
    for (index, depth) in depths.iter().take(depth_groups.len()).enumerate() {
        depth_groups[index] += depth;
    }

    let num_increases = ZipWithNext::new(depth_groups.iter())
        .filter(|(left, right)| left < right)
        .count();

    println!("{}", num_increases);
}
