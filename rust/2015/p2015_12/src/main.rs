use serde_json::Value;
use std::io::Read;

fn get_input_json() -> Value {
    let mut input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");
    let mut json_string = String::new();
    input_file.read_to_string(&mut json_string).expect("Unable to read input.");
    serde_json::from_str(&json_string).expect("Unable to parse JSON.")
}

fn get_sum_of_numbers(value: &Value, exclude_name: Option<&str>) -> i64 {
    match value {
        Value::Number(number) => number.as_i64().unwrap(),
        Value::Array(array) => {
            array.iter().fold(0, |sum, array_value| sum + get_sum_of_numbers(array_value, exclude_name))
        },
        Value::Object(map) => {
            if exclude_name.is_some() && map.values().any(|value| value == exclude_name.unwrap()) {
                return 0;
            }

            map.values().fold(0, |sum, map_value| sum + get_sum_of_numbers(map_value, exclude_name))
        },
        _ => 0,
    }
}

fn main() {
    println!("Sum: {}", get_sum_of_numbers(&get_input_json(), Some("red")));
}
