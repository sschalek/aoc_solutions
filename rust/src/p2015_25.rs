// Get the row and column to find the code for from the given string.
fn parse_coordinate(coordinate_string: &str) -> (i32, i32) {
    let integers = coordinate_string
        .split(|c: char| !c.is_numeric())
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<_>>();
    assert!(integers.len() == 2, "Unexpected coordinate string format");
    (integers[0], integers[1])
}

// 1 + 2 + 3 + ... + n = n * (n + 1) / 2
fn get_triangular_number(n: i32) -> i32 {
    n * (n + 1) / 2
}

// Get the index of the code at the given coordinate in the table of codes.
fn get_index_from_coordinate((row, col): (i32, i32)) -> i32 {
    // Calculate the index using the formula for partial sums of triangular numbers (1 + 2 + 3 + ... + n = n * (n + 1) / 2).
    // Each move to the next row or column increases the index by 1 more than the previous move.
    // Moving from (row, col) to (row + 1, col) increases the index by (col + row - 1).
    // Moving from (row, col) to (row, col + 1) increases the index by (col + row).
    // The indices for the first row are given by the sequence of triangular numbers.
    // The indices for column n are given by the index of the code at (1, n) plus (n + (n + 1) + (n + 2) + ... + (n + row - 2)).
    let col_contibution = get_triangular_number(col);
    let row_contribution = get_triangular_number(col + row - 2) - get_triangular_number(col - 1);
    col_contibution + row_contribution
}

// Get an iterator over the sequence of codes.
fn get_code_sequence_iterator() -> impl Iterator<Item = i64> {
    let mut code = 20151125;
    std::iter::from_fn(move || {
        let current_code = code;
        code = (code * 252_533) % 33_554_393;
        Some(current_code)
    })
}

// Get the code at the given coordinate in the table of codes.
fn get_code((row, col): (i32, i32)) -> i64 {
    let index = get_index_from_coordinate((row, col));
    get_code_sequence_iterator().nth(index as usize - 1).unwrap()
}

fn solve(input: &str, log_fn: Option<fn(&str)>) -> (String, String) {
    // Part 1: Find the code at the given coordinate in the table of codes.
    let input_coordinate = parse_coordinate(input);
    let code = get_code(input_coordinate);

    if let Some(log_fn) = log_fn {
        log_fn(&format!("Code for {input_coordinate:?}: {code}"));
    }

    (code.to_string(), String::new())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 25, solve);
