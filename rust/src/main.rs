#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::unreadable_literal,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]

use linkme::distributed_slice;
use std::collections::HashMap;
use std::io::prelude::Read;

mod p2015_01;
mod p2015_02;
mod p2015_03;
mod p2015_04;
mod p2015_05;
mod p2015_06;
mod p2015_07;
mod p2015_08;
//mod p2015_09;
mod p2015_10;
mod p2015_11;
mod p2015_12;
//mod p2015_13;
//mod p2015_14;
//mod p2015_15;
//mod p2015_16;
//mod p2015_17;
//mod p2015_18;
//mod p2015_19;
//mod p2015_20;
//mod p2015_21;
//mod p2015_22;
//mod p2015_23;
//mod p2015_24;
mod p2015_25;

fn get_input_path(year: i32, day: i32) -> String {
    format!("inputs/{year}/{day}.txt")
}

// Returns the path of and a file handle to the input file for the given year and day.
fn open_input_file(year: i32, day: i32) -> (String, std::fs::File) {
    let input_path = get_input_path(year, day);
    let input_file = std::fs::File::open(&input_path).unwrap_or_else(|_| {
        panic!("A file containing the problem input must be present at \"{input_path}\" in the current directory.")
    });
    (input_path, input_file)
}

fn get_problem_input(year: i32, day: i32) -> String {
    let (input_path, mut input_file) = open_input_file(year, day);
    let mut input_string = String::new();
    input_file
        .read_to_string(&mut input_string)
        .unwrap_or_else(|_| panic!("The input file at \"{input_path}\" could not be read."));
    input_string.trim().to_owned()
}

type SolveFn = fn(&str, Option<fn(&str)>) -> (String, String);

#[derive(Clone, Copy)]
pub struct Solution {
    year: i32,
    day: i32,
    solve: SolveFn,
}

impl Solution {
    pub const fn new(year: i32, day: i32, solve: SolveFn) -> Self {
        Self { year, day, solve }
    }
}

#[distributed_slice]
pub static SOLUTIONS: [Solution] = [..];

struct SolutionRunResult {
    part1_result: String,
    part2_result: String,
    duration: std::time::Duration,
}

impl SolutionRunResult {
    fn new(part1_result: String, part2_result: String, duration: std::time::Duration) -> Self {
        Self {
            part1_result,
            part2_result,
            duration,
        }
    }
}

fn generate_solution_set(year: Option<i32>, day: Option<i32>) -> HashMap<i32, Vec<Option<Solution>>> {
    let mut solution_set = HashMap::new();
    for solution in SOLUTIONS {
        if let Some(year) = year {
            if solution.year != year {
                continue;
            }

            if let Some(day) = day {
                if solution.day != day {
                    continue;
                }
            }
        }

        let day_index = solution.day as usize - 1;
        solution_set.entry(solution.year).or_insert_with(|| vec![None; 25])[day_index] = Some(*solution);
    }
    solution_set
}

fn run_solutions(
    year: Option<i32>,
    day: Option<i32>,
    log_fn: Option<fn(&str)>,
) -> HashMap<i32, Vec<Option<SolutionRunResult>>> {
    let solution_map = generate_solution_set(year, day);
    let mut result_map = HashMap::new();
    for (year, solutions) in solution_map {
        let mut result_vec = Vec::new();
        for solution in solutions {
            if let Some(solution) = solution {
                let problem_input = get_problem_input(year, solution.day);
                let start_time = std::time::Instant::now();
                let results = (solution.solve)(&problem_input, log_fn);
                let duration = start_time.elapsed();
                result_vec.push(Some(SolutionRunResult::new(results.0, results.1, duration)));
            } else {
                result_vec.push(None);
            }
        }
        result_map.insert(year, result_vec);
    }
    result_map
}

fn print_usage_and_exit() {
    println!("Usage: {} [<year>] [<day>] [-v]", std::env::args().next().unwrap());
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 4 {
        print_usage_and_exit();
    }

    let requested_year = args.get(1).and_then(|s| s.parse::<i32>().ok());
    let requested_day = args.get(2).and_then(|s| s.parse::<i32>().ok());

    // If verbose mode is enabled, then define a logging callback that will print out the given string.
    // This may be used by solutions to print out intermediate results.
    let log_fn: Option<fn(&str)> = if args.contains(&"-v".to_owned()) {
        Some(|s| println!("{s}"))
    } else {
        None
    };

    // Run the solutions for the requested year and/or day, or for all years and days if no specific year or day were requested.
    let result_map = run_solutions(requested_year, requested_day, log_fn);

    // Print out the results in table form, with one table per year.
    for (year, results) in result_map {
        println!("Year {year}");
        println!("Day  Part 1        Part 2           Time");
        println!("---  ------------  ------------     ----------");

        let mut total_duration = std::time::Duration::new(0, 0);
        for (day, result) in results.iter().enumerate() {
            if let Some(result) = result {
                println!(
                    "{:>3}  {:>12}  {:>12}  {:>4}.{:03}_{:03}s",
                    day + 1,
                    result.part1_result,
                    result.part2_result,
                    result.duration.as_secs(),
                    result.duration.subsec_millis(),
                    result.duration.as_micros() % 1000
                );

                total_duration += result.duration;
            } else {
                println!("{:>3}  {:>12}  {:>12}  {:>4}.{:03}_{:03}s", day, "-", "-", 0, 0, 0);
            }
        }

        println!("---  ------------  ------------     ----------");
        println!(
            "     {:>12}  {:>12}  {:>4}.{:03}_{:03}s",
            "-",
            "-",
            total_duration.as_secs(),
            total_duration.subsec_millis(),
            total_duration.as_micros() % 1000
        );
        println!();
    }
}
