use std::io::Read;

// Get the row and column to find the code for from the input file.
fn get_input_coordinate() -> (i32, i32) {
    let mut input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).expect("Unable to read input.");
    let mut input_lines = input_string.lines();
    let row = input_lines.next().unwrap().parse::<i32>().unwrap();
    let col = input_lines.next().unwrap().parse::<i32>().unwrap();
    (row, col)
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
        code = (code * 252533) % 33554393;
        Some(current_code)
    })
}

// Get the code at the given coordinate in the table of codes.
fn get_code((row, col): (i32, i32)) -> i64 {
    let index = get_index_from_coordinate((row, col));
    get_code_sequence_iterator().nth(index as usize - 1).unwrap()
}

fn main() {
    let input_coordinate = get_input_coordinate();
    let code = get_code(input_coordinate);
    println!("Code for {:?}: {}", input_coordinate, code);
}
