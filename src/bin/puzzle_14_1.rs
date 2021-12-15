use std::{collections::HashMap, fs::read_to_string};

use advent_of_code::ZipWithNextExt;

fn main() {
    let input = read_to_string("puzzle_14_input").unwrap();

    let (template, pairs) = input.split_once("\n\n").unwrap();
    let mut template = template.to_string();

    let pairs = pairs
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" -> ").unwrap();

            let input_0 = input.chars().nth(0).unwrap();
            let output_0 = output.chars().nth(0).unwrap();

            let output = format!("{}{}", input_0, output_0);

            (input.to_string(), output)
        })
        .collect::<HashMap<_, _>>();

    for _ in 0..10 {
        template = template
            .chars()
            .zip_with_next()
            .map(|(a, b)| {
                let key = format!("{}{}", a, b);
                pairs[&key].to_string()
            })
            .collect::<Vec<_>>()
            .join("")
            + template.chars().last().unwrap().to_string().as_str();
    }

    let mut char_map = HashMap::new();

    for char in template.chars() {
        let count = char_map.entry(char).or_insert(0);
        *count += 1;
    }

    let most_common = char_map.iter().max_by_key(|(_, count)| *count).unwrap();

    let least_common = char_map.iter().min_by_key(|(_, count)| *count).unwrap();

    let total = most_common.1 - least_common.1;

    assert_eq!(2223, total);

    println!("{:?}", total);
}
