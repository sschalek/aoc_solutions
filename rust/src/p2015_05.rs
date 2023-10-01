use rayon::prelude::*;

// Returns true if the given string is "nice" according to the first set of rules.
fn is_nice_string1(string: &&str) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const DISALLOWED_SUBSTRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

    let mut vowel_count: usize = 0;
    let mut has_doubled_letter = false;

    // Iterate through each character in the input string to test if it is "nice"
    // according to the first set of rules, keeping track of a single previous character
    // to allow the rules to be checked in a single iteration.
    let mut last_char = '\0';
    for (i, c) in string.char_indices() {
        if VOWELS.contains(&c) {
            vowel_count += 1;
        }

        if i > 0 {
            // If this is not the first character in the string, check whether it matches the
            // previous character. If it does, mark that the string contains a doubled letter.
            if c == last_char {
                has_doubled_letter = true;
            }

            // Check whether a string made up of the current character and the last character
            // is in the list of disallowed two character strings and immediately return false
            // if it is.
            let last_string: &str = &[last_char, c].iter().collect::<String>();
            if DISALLOWED_SUBSTRINGS.contains(&last_string) {
                return false;
            };
        }

        last_char = c;
    }

    // Return true if the string contains at least three vowels and a doubled letter.
    (vowel_count >= 3) && has_doubled_letter
}

// Returns true if the given string is "nice" according to the second set of rules.
fn is_nice_string2(string: &&str) -> bool {
    let mut found_matching_pair = false;
    let mut found_repeated_letter = false;
    let mut last_char = '\0';
    let mut last_last_char = '\0';
    for (i, c) in string.char_indices() {
        // Check whether the given string contains two matching, non-overlapping pairs of characters.
        if (i < (string.len() - 2)) && (i > 0) && string[(i + 1)..].contains(&[last_char, c].iter().collect::<String>())
        {
            found_matching_pair = true;
        }

        // Check whether the given string contains a doubled letter.
        if i > 1 && c == last_last_char {
            found_repeated_letter = true;
        }

        last_last_char = last_char;
        last_char = c;
    }

    found_matching_pair && found_repeated_letter
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    // Part 1: Find the number of "nice" strings in the input list according to the first set of rules.
    let part1_result = input.par_lines().filter(is_nice_string1).collect::<Vec<&str>>().len();

    // Part 2: Find the number of "nice" strings in the input list according to the second set of rules.
    let part2_result = input.par_lines().filter(is_nice_string2).collect::<Vec<&str>>().len();

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 5, solve);
