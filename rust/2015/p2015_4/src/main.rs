extern crate md5;
use md5::Digest;
use std::convert::TryInto;

fn md5_hash_key_and_num(key: &String, num: usize) -> [u8; 16] {
    let mut hasher = md5::Md5::new();
    hasher.update(key.as_bytes());
    hasher.update(num.to_string().as_bytes());
    return hasher.finalize()[..].try_into().unwrap();
}

fn is_desired_md5_hash(hash: &[u8; 16], leading_zero_count: usize) -> bool {
    return
        hash[0..(leading_zero_count / 2)].iter().all(|b| *b == 0) &&
        hash[leading_zero_count / 2 + leading_zero_count % 2 - 1] & 0xF0 == 0;
}

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
    println!("{}", find_first_zero_prefixed_hash_number(&key, leading_zero_count.parse::<usize>().unwrap()));
}
