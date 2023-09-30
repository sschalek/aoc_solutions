#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::io::Read;

// Returns the contents of the input file as a string.
fn get_instructions_string() -> String {
    let mut input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");
    let mut instructions_string = String::new();
    input_file
        .read_to_string(&mut instructions_string)
        .expect("Unable to read input.");
    instructions_string
}

fn apply_look_and_say(input: &str) -> String {
    let mut output_string = String::new();
    let mut current_number: Option<char> = None;
    let mut current_number_count = 0;
    for c in input.chars() {
        // If the current number is different from the previous number, output the previous number and its count.
        if current_number.is_none() || current_number.unwrap() != c {
            // If there is a current number, output it and its count.
            if let Some(some_current_number) = current_number {
                output_string.push_str(&current_number_count.to_string());
                output_string.push(some_current_number);
            }

            // Reset the current number and count.
            current_number = Some(c);
            current_number_count = 1;
        } else {
            current_number_count += 1;
        }
    }

    // Output the last number and its count.
    output_string.push_str(&current_number_count.to_string());
    output_string.push(current_number.unwrap());
    output_string
}

fn main() {
    let apply_count = 50;
    let mut current_string = get_instructions_string();
    for i in 0..apply_count {
        current_string = apply_look_and_say(&current_string);

        if i < 10 {
            println!("String after {} iterations: {}", i + 1, current_string);
        } else {
            println!(
                "Length of string after {} iterations: {}",
                i + 1,
                current_string.len()
            );
        }
    }
}
