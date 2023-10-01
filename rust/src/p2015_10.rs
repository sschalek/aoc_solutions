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

fn solve(input: &str, log_fn: Option<fn(&str)>) -> (String, String) {
    // Part 1 and 2: Find the length of the string after 40 and 50 iterations of the look-and-say algorithm.
    let mut length_after_40_iterations = 0;
    let mut length_after_50_iterations = 0;

    let apply_count = 50;
    let mut current_string = input.to_owned();
    for i in 0..apply_count {
        current_string = apply_look_and_say(&current_string);

        if let Some(log_fn) = log_fn {
            if i < 10 {
                log_fn(&format!("String after {} iterations: {}", i + 1, current_string));
            } else {
                log_fn(&format!(
                    "Length of string after {} iterations: {}",
                    i + 1,
                    current_string.len()
                ));
            }
        }

        if i == 40 - 1 {
            length_after_40_iterations = current_string.len();
        } else if i == 50 - 1 {
            length_after_50_iterations = current_string.len();
        }
    }

    (
        length_after_40_iterations.to_string(),
        length_after_50_iterations.to_string(),
    )
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 10, solve);
