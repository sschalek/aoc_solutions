// Advent of Code 2015, Day 16: "Aunt Sue"
// https://adventofcode.com/2015/day/16

use std::collections::HashMap;

// Represents the characteristics of an Aunt Sue.
#[derive(Clone, Debug)]
struct SueDescription {
    characteristics: HashMap<String, u32>,
}

// Parses a string into a SueDescription.
fn parse_sue_description(line: &str) -> SueDescription {
    let mut characteristics = HashMap::new();
    let characteristic_strings = line.split_once(": ").unwrap().1.split(", ");
    for characteristic_string in characteristic_strings {
        let mut characteristic = characteristic_string.split(": ");
        let characteristic_name = characteristic.next().unwrap().to_string();
        let characteristic_value = characteristic.next().unwrap().parse::<u32>().unwrap();
        characteristics.insert(characteristic_name, characteristic_value);
    }
    SueDescription { characteristics }
}

// Parses the given string into a vector of SueDescriptions.
fn parse_sue_descriptions(input: &str) -> Vec<SueDescription> {
    input.lines().map(parse_sue_description).collect()
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    let sue_desciptions = parse_sue_descriptions(input);

    let measured_sue_characteristics: HashMap<String, u32> = [
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]
    .iter()
    .cloned()
    .collect();

    // Part 1: Find the Sue that matches the given characteristics.
    let mut matching_sues = sue_desciptions.iter().enumerate().filter(|(_, sue_description)| {
        sue_description
            .characteristics
            .iter()
            .all(|(characteristic_name, characteristic_value)| {
                measured_sue_characteristics.get(characteristic_name).unwrap() == characteristic_value
            })
    });
    let part1_result = matching_sues.next().unwrap().0 + 1;

    // Part 2: Find the Sue that matches the given characteristics, under the condition that the
    // measured characteristics are less than the actual characteristics for "cats" and "trees" and
    // greater than the actual characteristics for "pomeranians" and "goldfish".
    #[allow(clippy::items_after_statements)]
    const GREATER_THAN_CHARACTERISTICS: [&str; 2] = ["cats", "trees"];
    #[allow(clippy::items_after_statements)]
    const LESS_THAN_CHARACTERISTICS: [&str; 2] = ["pomeranians", "goldfish"];

    let mut matching_sues = sue_desciptions.iter().enumerate().filter(|(_, sue_description)| {
        sue_description
            .characteristics
            .iter()
            .all(|(characteristic_name, characteristic_value)| {
                let measured_characteristic_value = measured_sue_characteristics.get(characteristic_name).unwrap();
                if GREATER_THAN_CHARACTERISTICS.contains(&characteristic_name.as_str()) {
                    measured_characteristic_value < characteristic_value
                } else if LESS_THAN_CHARACTERISTICS.contains(&characteristic_name.as_str()) {
                    measured_characteristic_value > characteristic_value
                } else {
                    measured_characteristic_value == characteristic_value
                }
            })
    });
    let part2_result = matching_sues.next().unwrap().0 + 1;

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 16, solve);
