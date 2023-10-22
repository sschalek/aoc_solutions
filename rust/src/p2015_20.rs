// Advent of Code 2015, Day 20: "Infinite Elves and Infinite Houses"
// https://adventofcode.com/2015/day/20

use std::collections::{HashMap, HashSet};

// TODO: This solution is much slower than the others, which generally take less than a second.
//       Come back to this and look for a better approach. It might be that things can be improved
//       by reusing partial results from previous iterations when determining factors.

// Represents a generator that can be iterated over to produce the number of presents delivered to
// each house given the multiplier for the number of presents delivered by each elf and the limit
// on the number of houses that each elf can deliver to.
struct PresentCountGenerator<'a> {
    house_number: u64,
    present_multiplier: u64,
    house_limit: Option<u64>,
    #[allow(unused)]
    factor_cache: &'a mut HashMap<u64, HashSet<u64>>,
}

impl PresentCountGenerator<'_> {
    // Creates a new PresentCountGenerator with the given present multiplier and house limit.
    fn new(
        present_multipler: u64,
        house_limit: Option<u64>,
        factor_cache: &mut HashMap<u64, HashSet<u64>>,
    ) -> PresentCountGenerator {
        PresentCountGenerator {
            house_number: 0,
            present_multiplier: present_multipler,
            house_limit,
            factor_cache,
        }
    }

    // Returns a vector of all factors of the given number.
    fn get_all_factors(n: u64) -> Vec<u64> {
        // Set up the vector that will hold all of the factors of the given number.
        // The first factor is always 1, and the last factor is always the number itself.
        let mut factors = Vec::new();
        factors.push(1);
        factors.push(n);

        // Check each number starting at 2 up to the square root of the given number
        // to see if it is a factor. If it is, then add it to the list of factors.
        // Only check up to the square root because any factor larger than the square
        // root will have a corresponding factor that is smaller than the square root.
        let mut i = 2;
        while i * i <= n {
            // Caclulate the potential "opposite factor" of the current factor. For example, if the
            // current factor is 2 and the given number is 10, then the opposite factor is 5.
            // Check to see if the integer factors multiply together to equal the given number. If
            // they do, then the current potential factor and its opposite factor are both factors
            // of the given number, so add them both to the list of factors.
            let opposite_factor = n / i;
            if opposite_factor * i == n {
                factors.push(i);
                if opposite_factor != i {
                    factors.push(opposite_factor);
                }
            }
            i += 1;
        }
        factors
    }
}

// Implement the Iterator trait for PresentCountGenerator to enable iterating over it
// to produce the number of presents delivered to each house.
impl Iterator for PresentCountGenerator<'_> {
    type Item = u64;

    // Returns the next number of presents delivered to a house.
    fn next(&mut self) -> Option<Self::Item> {
        // Increment the house number and calculate the number of presents delivered to this house
        // by summing the number of presents delivered by each elf associated with a factor of the
        // house number.
        self.house_number += 1;
        let mut present_count = 0;
        for factor in Self::get_all_factors(self.house_number) {
            // If there is no house limit, or if the house limit is not exceeded, then add the
            // number of presents delivered by the elf associated with this factor to the total
            // number of presents delivered to this house.
            if self.house_limit.is_none() || self.house_number <= self.house_limit.unwrap() * factor {
                present_count += factor * self.present_multiplier;
            }
        }
        Some(present_count)
    }
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    let minimum_presents = input.parse::<u64>().unwrap();

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
