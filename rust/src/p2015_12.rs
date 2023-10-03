// Advent of Code 2015, Day 12: "JSAbacusFramework.io"
// https://adventofcode.com/2015/day/12

use serde_json::Value;

fn get_sum_of_numbers(value: &Value, exclude_name: Option<&str>) -> i64 {
    match value {
        Value::Number(number) => number.as_i64().unwrap(),
        Value::Array(array) => array.iter().fold(0, |sum, array_value| {
            sum + get_sum_of_numbers(array_value, exclude_name)
        }),
        Value::Object(map) => {
            if exclude_name.is_some() && map.values().any(|value| value == exclude_name.unwrap()) {
                return 0;
            }

            map.values()
                .fold(0, |sum, map_value| sum + get_sum_of_numbers(map_value, exclude_name))
        }
        _ => 0,
    }
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    let input_json: Value = serde_json::from_str(input).expect("Unable to parse JSON.");

    // Part 1: Find the sum of all numbers in the JSON.
    let part1_result = get_sum_of_numbers(&input_json, None);

    // Part 2: Find the sum of all numbers in the JSON, excluding any object that has a value
    // with the name "red".
    let part2_result = get_sum_of_numbers(&input_json, Some("red"));

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 12, solve);
