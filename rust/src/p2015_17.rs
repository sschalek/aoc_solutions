// Advent of Code 2015, Day 17: "No Such Thing as Too Much"
// https://adventofcode.com/2015/day/17

use std::collections::HashMap;

fn get_fill_combinations(container_sizes: &[u32], target_volume: u32, log_fn: Option<fn(&str)>) -> (u32, u32) {
    fn get_fill_combinations_recursive(
        sorted_remaining_container_sizes: &[u32],
        target_volume: u32,
        total_fill_combinations: &mut HashMap<usize, u32>,
        combination_count_cache: &mut HashMap<(Vec<u32>, u32), HashMap<usize, u32>>,
        _log_fn: Option<fn(&str)>,
    ) {
        if target_volume == 0 {
            total_fill_combinations.entry(1).and_modify(|c| *c += 1).or_insert(1);
            return;
        }

        for (i, container_size) in sorted_remaining_container_sizes.iter().enumerate() {
            if target_volume < *container_size {
                continue;
            }

            let remaining_volume = target_volume - container_size;
            let remaining_container_sizes = sorted_remaining_container_sizes[i + 1..].to_vec();
            let mut fill_combinations = HashMap::new();

            if let Some(cached_combinations) =
                combination_count_cache.get(&(remaining_container_sizes.clone(), remaining_volume))
            {
                fill_combinations = cached_combinations.clone();
            } else {
                get_fill_combinations_recursive(
                    &remaining_container_sizes,
                    remaining_volume,
                    &mut fill_combinations,
                    combination_count_cache,
                    #[allow(clippy::used_underscore_binding)]
                    _log_fn,
                );

                combination_count_cache.insert(
                    (remaining_container_sizes.clone(), remaining_volume),
                    fill_combinations.clone(),
                );
            }

            for (combination_size, combination_count) in &fill_combinations {
                total_fill_combinations
                    .entry(*combination_size + 1)
                    .and_modify(|c| *c += combination_count)
                    .or_insert(*combination_count);
            }
        }
    }

    let mut sorted_remaining_container_sizes = container_sizes.to_vec();
    sorted_remaining_container_sizes.sort_unstable();
    let mut fill_combinations = HashMap::new();
    let mut combination_count_cache = HashMap::new();
    get_fill_combinations_recursive(
        &sorted_remaining_container_sizes,
        target_volume,
        &mut fill_combinations,
        &mut combination_count_cache,
        log_fn,
    );

    if let Some(log_fn) = log_fn {
        log_fn("Fill combinations:");
        for (container_count, combination_count) in &fill_combinations {
            log_fn(&format!("{container_count:>4}: {combination_count}"));
        }
    }

    let mut total_container_combination_count = 0;
    let mut min_container_count = 0;
    let mut min_container_count_combination_count = 0;
    for (container_count, combination_count) in &fill_combinations {
        if min_container_count == 0 || *container_count < min_container_count {
            min_container_count = *container_count;
            min_container_count_combination_count = *combination_count;
        }

        total_container_combination_count += combination_count;
    }
    (total_container_combination_count, min_container_count_combination_count)
}

fn parse_container_sizes(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse::<u32>().unwrap()).collect()
}

fn solve(input: &str, log_fn: Option<fn(&str)>) -> (String, String) {
    let container_sizes = parse_container_sizes(input);

    // Part 1: Find the number of ways to fill the containers with 150 liters of eggnog.
    // Part 2: Find the number of ways to fill the containers with 150 liters of eggnog using the
    // minimum number of containers.
    let (part1_result, part2_result) = get_fill_combinations(&container_sizes, 150, log_fn);

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 17, solve);
