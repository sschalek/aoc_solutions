// Advent of Code 2015, Day 4: "The Ideal Stocking Stuffer"
// https://adventofcode.com/2015/day/4

use md5::Digest;
use rayon::prelude::*;
use std::convert::TryInto;

// Returns the MD5 hash of the string resulting from appending the string representation of the given number
// to the given key string. The hash is returned as a 16 byte slice.
fn md5_hash_key_and_num(key: &str, num: usize) -> [u8; 16] {
    let mut hasher = md5::Md5::new();
    hasher.update(key.as_bytes());
    hasher.update(num.to_string().as_bytes());
    hasher.finalize()[..].try_into().unwrap()
}

// Returns whether the hex representation of the given 16 byte slice hash has at least the given
// leading_zero_count.
fn is_desired_md5_hash(hash: &[u8; 16], leading_zero_count: usize) -> bool {
    // Since each byte translates to two digits in the hexadecimal representation of the 16 byte slice,
    // only leading_zero_count / 2 leading bytes need to be 0. If an odd number of leading zeros is desired,
    // then only the first half of the last byte checked needs to be 0.
    hash[0..(leading_zero_count / 2)].iter().all(|b| *b == 0)
        && hash[leading_zero_count / 2 + leading_zero_count % 2 - 1] & 0xF0 == 0
}

// Returns the first number that results in an MD5 hash whose hexadecimal representation has leading_zero_count
// leading zeroes when its string representation is appended to the given key string and the resulting string is
// hashed.
fn find_first_zero_prefixed_hash_number(key: &str, leading_zero_count: usize) -> usize {
    // Iterate through chunks of numbers, hashing each number in parallel until a number is found that results
    // in a hash with the desired number of leading zeroes.
    let chunk_size = 100000;
    let mut chunk_begin = 0;
    loop {
        let chunk_end = chunk_begin + chunk_size;
        let chunk_result = (chunk_begin..chunk_end)
            .into_par_iter()
            .map(|n| (n, md5_hash_key_and_num(key, n)))
            .find_first(|(_, hash)| is_desired_md5_hash(hash, leading_zero_count));

        if let Some((n, _)) = chunk_result {
            return n;
        }

        chunk_begin = chunk_end;
    }
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    // Part 1: Find the first number that results in a hash with five leading zeros when
    // its string representation is appended to the given key string.
    let part1_result = find_first_zero_prefixed_hash_number(input, 5);

    // Part 2: Find the first number that results in a hash with six leading zeros when
    // its string representation is appended to the given key string.
    let part2_result = find_first_zero_prefixed_hash_number(input, 6);

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 4, solve);
