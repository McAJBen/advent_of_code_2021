use advent_of_code::ZipWithNextExt;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("puzzle_14_input").unwrap();

    let (template, pairs) = input.split_once("\n\n").unwrap();

    let template_start = *template.chars().collect::<Vec<_>>().first().unwrap();
    let template_end = *template.chars().collect::<Vec<_>>().last().unwrap();

    let mut template = template
        .chars()
        .zip_with_next()
        .map(|(a, b)| format!("{}{}", a, b))
        .fold(HashMap::new(), |mut acc, key| {
            *acc.entry(key).or_insert(0u128) += 1;
            acc
        });

    let rules = pairs
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" -> ").unwrap();

            let input_0 = input.chars().next().unwrap();
            let output_0 = output.chars().next().unwrap();
            let input_1 = input.chars().nth(1).unwrap();

            let output = vec![
                format!("{}{}", input_0, output_0),
                format!("{}{}", output_0, input_1),
            ];

            (input.to_string(), output)
        })
        .collect::<HashMap<_, _>>();

    for _ in 0..40 {
        let mut new_template = HashMap::new();

        for (key, count) in template {
            for output in rules[&key].iter() {
                *new_template.entry(output.clone()).or_insert(0) += count;
            }
        }
        template = new_template;
    }

    let mut char_map = HashMap::new();

    *char_map.entry(template_start).or_insert(0) += 1;
    *char_map.entry(template_end).or_insert(0) += 1;

    for (key, count) in template {
        for char in key.chars() {
            *char_map.entry(char).or_insert(0) += count;
        }
    }

    let most_common = char_map.iter().max_by_key(|(_, count)| *count).unwrap();

    let least_common = char_map.iter().min_by_key(|(_, count)| *count).unwrap();

    let total = (most_common.1 - least_common.1) / 2;

    assert_eq!(2566282754493, total);

    println!("{}", total);
}
