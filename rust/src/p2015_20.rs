// Advent of Code 2015, Day 20: "Infinite Elves and Infinite Houses"
// https://adventofcode.com/2015/day/20

use std::collections::{HashMap, HashSet};

struct PresentCountGenerator<'a> {
    house_number: u64,
    present_multiplier: u64,
    house_limit: Option<u64>,
    factor_cache: &'a mut HashMap<u64, HashSet<u64>>,
}

impl PresentCountGenerator<'_> {
    fn new(
        present_multipler: u64,
        house_limit: Option<u64>,
        factor_cache: &mut HashMap<u64, HashSet<u64>>,
    ) -> PresentCountGenerator {
        PresentCountGenerator {
            house_number: 500000,
            present_multiplier: present_multipler,
            house_limit,
            factor_cache,
        }
    }

    fn get_all_factors(&mut self, n: u64) -> HashSet<u64> {
        fn get_all_factors_recursive(
            n: u64,
            factors: &mut HashSet<u64>,
            factor_cache: &mut HashMap<u64, HashSet<u64>>,
        ) {
            factors.insert(1);
            factors.insert(n);

            let mut i = 2;
            while i * i <= n {
                let opposite_factor = n / i;
                if opposite_factor * i == n {
                    factors.insert(i);

                    if let Some(cached_factors) = factor_cache.get(&opposite_factor) {
                        factors.extend(cached_factors.iter());
                    } else {
                        let mut new_factors = HashSet::new();
                        get_all_factors_recursive(opposite_factor, &mut new_factors, factor_cache);
                        factor_cache.insert(opposite_factor, new_factors.clone());
                        factors.extend(new_factors.iter());
                    }
                }
                i += 1;
            }
        }

        let mut factors = HashSet::new();
        get_all_factors_recursive(n, &mut factors, self.factor_cache);
        factors
    }
}

impl Iterator for PresentCountGenerator<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.house_number += 1;
        let mut present_count = 0;
        for factor in self.get_all_factors(self.house_number) {
            if self.house_limit.is_none() || self.house_number / factor <= self.house_limit.unwrap() {
                present_count += factor * self.present_multiplier;
            }
        }
        //println!("{}: {}", self.house_number, present_count);
        Some(present_count)
    }
}

fn parse_minimum_presents(input: &str) -> u64 {
    input.parse::<u64>().unwrap()
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    let minimum_presents = parse_minimum_presents(input);

    let mut factor_cache = HashMap::new();

    // Part 1: Find the first house to receive at least the given number of presents.
    let mut present_count_generator = PresentCountGenerator::new(10, None, &mut factor_cache);
    let minimum_house_index = present_count_generator
        .position(|present_count| present_count >= minimum_presents)
        .unwrap() as u64;
    let part1_result = minimum_house_index + 1;

    // Part 2: Find the first house to receive at least the given number of presents, where each
    // elf only delivers presents to 50 houses.
    let mut present_count_generator = PresentCountGenerator::new(11, Some(50), &mut factor_cache);
    let minimum_house_index = present_count_generator
        .position(|present_count| present_count >= minimum_presents)
        .unwrap() as u64;
    let part2_result = minimum_house_index + 1;

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 20, solve);

// struct PresentCountGenerator {
//     factor_counters: Vec<u64>,
//     house_number: u64,
//     house_limit: Option<u64>,
//     present_multiplier: u64,
// }

// impl PresentCountGenerator {
//     fn new(house_limit: Option<u64>, present_multiplier: u64) -> PresentCountGenerator {
//         PresentCountGenerator {
//             factor_counters: vec![0],
//             house_number: 0,
//             house_limit,
//             present_multiplier,
//         }
//     }
// }

// impl Iterator for PresentCountGenerator {
//     type Item = u64;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.house_number += 1;
//         self.factor_counters.push(self.factor_counters.len() as u64);

//         let mut present_count = 0;
//         for (i, factor_counter) in self.factor_counters.iter_mut().enumerate() {
//             if *factor_counter == i as u64 {
//                 *factor_counter = 0;

//                 if self.house_limit.is_none() || self.house_number / i as u64 <= self.house_limit.unwrap() {
//                     present_count += (i as u64) * self.present_multiplier;
//                 }
//             } else {
//                 *factor_counter += 1;
//             }
//         }
//         if self.house_number % 10000 == 0 {
//             println!("{}: {}", self.house_number, present_count);
//         }
//         Some(present_count)
//     }
// }

// fn parse_minimum_presents(input: &str) -> u64 {
//     input.parse::<u64>().unwrap()
// }

// fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
//     let minimum_presents = parse_minimum_presents(input);

//     // Part 1: Find the first house to receive at least the given number of presents.
//     let mut present_count_generator = PresentCountGenerator::new(None, 10);
//     let minimum_house_index = present_count_generator
//         .position(|present_count| present_count >= minimum_presents)
//         .unwrap() as u64;
//     let part1_result = minimum_house_index + 1;

//     // Part 2: Find the first house to receive at least the given number of presents, where each
//     // elf only delivers presents to 50 houses.
//     let mut present_count_generator = PresentCountGenerator::new(Some(50), 11);
//     let minimum_house_index = present_count_generator
//         .position(|present_count| present_count >= minimum_presents)
//         .unwrap() as u64;
//     let part2_result = minimum_house_index + 1;

//     (part1_result.to_string(), part2_result.to_string())
// }

// #[linkme::distributed_slice(crate::SOLUTIONS)]
// static SOLUTION: crate::Solution = crate::Solution::new(2015, 20, solve);
