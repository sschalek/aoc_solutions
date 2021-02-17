extern crate md5;
use md5::Digest;
use std::convert::TryInto;

// Returns the MD5 hash of the string resulting from appending the string representation of the given number
// to the given key string. The hash is returned as a 16 byte slice.
fn md5_hash_key_and_num(key: &String, num: usize) -> [u8; 16] {
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
    hash[0..(leading_zero_count / 2)].iter().all(|b| *b == 0) &&
    hash[leading_zero_count / 2 + leading_zero_count % 2 - 1] & 0xF0 == 0
}

// Returns the first number that results in an MD5 hash whose hexadecimal representation has leading_zero_count
// leading zeroes when its string representation is appended to the given key string and the resulting string is
// hashed.
fn find_first_zero_prefixed_hash_number(key: &String, leading_zero_count: usize) -> usize {
    let mut test_number = 0;
    loop {
        if is_desired_md5_hash(&md5_hash_key_and_num(key, test_number), leading_zero_count) {
            return test_number;
        }

        test_number += 1;
    }
}

fn main() {
    let key = std::env::args().nth(1).expect("The key must be provided as the first command line argument.");
    let leading_zero_count = std::env::args().nth(2).expect("The desired leading zero count must be provided as the second command line argument.");

    // Part 1 & 2: Print out the first number that results in a hash with the given leading zero count when
    // its string representation is appended to the given key string.
    println!("{}", find_first_zero_prefixed_hash_number(&key, leading_zero_count.parse::<usize>().unwrap()));
}
