struct PasswordIterator {
    password: Vec<char>,
    invalid_characters: Vec<char>,
}

impl PasswordIterator {
    pub fn new(password: &str, invalid_characters: &[char]) -> Self {
        Self {
            password: password.chars().collect(),
            invalid_characters: invalid_characters.to_vec(),
        }
    }

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

impl Iterator for PasswordIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut password = self.password.clone();
        let mut i = password.len() - 1;
        loop {
            let (next_character, carry) = self.get_next_character(password[i]);
            password[i] = next_character;
            if !carry {
                break;
            }

            if i == 0 {
                return None;
            }
            i -= 1;
        }
        self.password = password;
        Some(self.password.iter().collect())
    }
}

fn is_valid_password(password: &str, invalid_characters: &[char]) -> bool {
    let mut has_increasing_straight = false;
    let mut has_two_pairs = false;

    let mut last_char = '\0';
    let mut last_last_char = '\0';
    let mut first_pair_char = '\0';
    for (i, c) in password.char_indices() {
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

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    const INVALID_CHARACTERS: [char; 3] = ['i', 'o', 'l'];

    // Part 1: Find the next valid password after the input password.
    let mut password_iterator = PasswordIterator::new(input, &INVALID_CHARACTERS);
    let part1_result = password_iterator
        .find(|p| is_valid_password(p, &INVALID_CHARACTERS))
        .unwrap();

    // Part 2: Find the next valid password after the part 1 result.
    let part2_result = password_iterator
        .find(|p| is_valid_password(p, &INVALID_CHARACTERS))
        .unwrap();

    (part1_result, part2_result)
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 11, solve);
