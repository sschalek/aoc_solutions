// Advent of Code 2015, Day 1: "Not Quite Lisp"
// https://adventofcode.com/2015/day/1

// Started with this solution, but the fold/reduce-based solution below seems to express
// things more clearly.
// fn get_floor_old(instruction_str:&str) -> Result<i64, ()> {
//     let mut floor_number:i64 = 0;
//     for c in instruction_str.chars() {
//         if c == '(' {
//             floor_number += 1;
//         }
//         else if c == ')' {
//             floor_number -= 1;
//         }
//         else {
//             return Result::Err(());
//         }
//     }
//     return Result::Ok(floor_number);
// }

// Given an instruction_str where '(' means go up a floor and ')' means go down
// a floor, returns a result containing the final floor number after following
// the instructions if the string is valid and () otherwise.
fn get_floor_number(instruction_str: &str) -> Result<i64, ()> {
    // Fold the given string into a single floor number, accumulating
    // the floor number and incrementing it or decrementing it for each character.
    instruction_str.chars().try_fold(0, |floor_number, c| match c {
        '(' => Ok(floor_number + 1),
        ')' => Ok(floor_number - 1),
        _ => Err(()),
    })
}

// Given the instruction_str where '(' means go up a floor and ')' means go down
// a floor, returns a result containing the index of the first instruction character
// to result in a basement floor being reached if the string is valid and () otherwise.
fn get_first_basement_char(instruction_str: &str) -> Result<usize, ()> {
    // Go through the given input string, updating the current floor number
    // based on each character, until a below-ground floor is first reached.
    let mut floor_number: isize = 0;
    for (i, c) in instruction_str.char_indices() {
        floor_number += match c {
            '(' => Ok(1),
            ')' => Ok(-1),
            _ => Err(()),
        }?;

        if floor_number < 0 {
            return Ok(i + 1);
        }
    }

    Err(())
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    // Part 1: Find the resulting floor number after following the instructions.
    let part1_result = get_floor_number(input).unwrap();

    // Part 2: Find the index of the first instruction character to result in a basement level
    // being reached.
    let part2_result = get_first_basement_char(input).unwrap();

    (part1_result.to_string(), part2_result.to_string())
}

// Register the solution via linkme.
#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 1, solve);
