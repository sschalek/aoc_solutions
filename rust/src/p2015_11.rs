// Advent of Code 2015, Day 11: "Corporate Policy"
// https://adventofcode.com/2015/day/11

// This struct implements an iterator over valid passwords, given a starting password and a set of invalid characters.
struct PasswordIterator {
    password: Vec<char>,
    invalid_characters: Vec<char>,
}

impl PasswordIterator {
    // Creates a new PasswordIterator instance with the given starting password and invalid characters.
    pub fn new(password: &str, invalid_characters: &[char]) -> Self {
        Self {
            password: password.chars().collect(),
            invalid_characters: invalid_characters.to_vec(),
        }
    }

    // Given a character, return the next valid password character and whether a
    // carry/wraparound occurred.
    fn get_next_character(&mut self, c: char) -> (char, bool) {
        let mut next_character = c;
        let mut carry = false;
        loop {
            if next_character == 'z' {
                next_character = 'a';
                carry = true;
            } else {
                next_character = (next_character as u8 + 1) as char;
            }

            if !self.invalid_characters.contains(&next_character) {
                break;
            }
        }
        (next_character, carry)
    }
}

// Returns true if the given password is valid according to the rule set and false otherwise.
// In addition to the built-in rule set, the password may not contain any of the given invalid characters.
fn is_valid_password(password: &[char], invalid_characters: &[char]) -> bool {
    let mut has_increasing_straight = false;
    let mut has_two_pairs = false;

    // Iterate over the characters in the given password, keeping track of the last two characters seen.
    let mut last_char = '\0';
    let mut last_last_char = '\0';
    let mut first_pair_char = '\0';
    for (i, c_ref) in password.iter().enumerate() {
        let c = *c_ref;

        // Check whether the given string contains any invalid characters.
        if invalid_characters.contains(&c) {
            return false;
        }

        // Check whether the given string contains a straight of three increasing characters.
        if i > 1 && (c as u8) == ((last_char as u8) + 1) && ((last_char as u8) == ((last_last_char as u8) + 1)) {
            has_increasing_straight = true;
        }

        // Check whether the given string contains two non-overlapping pairs of characters.
        if i > 0 && c == last_char {
            if first_pair_char == '\0' {
                first_pair_char = c;
            } else if c != first_pair_char {
                has_two_pairs = true;
            }
        }

        last_last_char = last_char;
        last_char = c;
    }

    has_increasing_straight && has_two_pairs
}

// Implement the standard Iterator trait for PasswordIterator.
impl Iterator for PasswordIterator {
    type Item = String;

    // Returns the next password after the current password, skipping any
    // passwords that contain invalid characters.
    fn next(&mut self) -> Option<Self::Item> {
        let mut password = self.password.clone();

        loop {
            // Increment the least significant character until it is valid.
            // If a carry occurs, increment the next most significant character.
            // Continue until no carry occurs or a carry occurs on the most significant character.
            let mut i = password.len() - 1;
            loop {
                // Get the next valid password character and whether a carry occurred.
                // Update the current password character to the next valid password character,
                // and check whether the next most significant character should be incremented.
                let (next_character, carry) = self.get_next_character(password[i]);
                password[i] = next_character;
                if !carry {
                    break;
                }

                // If a carry occurred on the most significant character, return None,
                // since there is no next valid password.
                if i == 0 {
                    return None;
                }
                // Otherwise, continue to the next most significant character.
                i -= 1;
            }

            // If the current password is valid, return it.
            if is_valid_password(&password, &self.invalid_characters) {
                break;
            }
        }

        self.password = password;
        Some(self.password.iter().collect())
    }
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    const INVALID_CHARACTERS: [char; 3] = ['i', 'o', 'l'];

    // Part 1: Find the next valid password after the input password.
    let mut password_iterator = PasswordIterator::new(input, &INVALID_CHARACTERS);
    let part1_result = password_iterator.next().unwrap();

    // Part 2: Find the next valid password after the part 1 result.
    let part2_result = password_iterator.next().unwrap();

    (part1_result, part2_result)
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 11, solve);
