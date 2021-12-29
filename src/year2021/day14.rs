use crate::utils::{read_input, ZipWithNextExt};
use std::collections::HashMap;

pub fn part1() -> i32 {
    let input = read_input(2021, 14);

    let (template, pairs) = input.split_once("\n\n").unwrap();
    let mut template = template.to_string();

    let pairs = pairs
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" -> ").unwrap();

            let input_0 = input.chars().next().unwrap();
            let output_0 = output.chars().next().unwrap();

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

    most_common.1 - least_common.1
}

pub fn part2() -> u64 {
    let input = read_input(2021, 14);

    let (template, pairs) = input.split_once("\n\n").unwrap();

    let template_start = *template.chars().collect::<Vec<_>>().first().unwrap();
    let template_end = *template.chars().collect::<Vec<_>>().last().unwrap();

    let mut template = template
        .chars()
        .zip_with_next()
        .map(|(a, b)| format!("{}{}", a, b))
        .fold(HashMap::new(), |mut acc, key| {
            *acc.entry(key).or_insert(0u64) += 1;
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

    (most_common.1 - least_common.1) / 2
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 2223);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 2566282754493);
}
