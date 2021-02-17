use std::io::BufRead;

// Returns an iterator that iterates through each line of the input file.
fn input_lines() -> impl Iterator<Item=String> {
    let input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");

    // Create a line-based iterator for the file contents.
    let reader = std::io::BufReader::new(input_file);
    reader.lines().map(|l| l.unwrap())
}

// Returns true if the given string is "nice" according to the first set of rules.
fn is_nice_string1(string: &String) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const DISALLOWED_SUBSTRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

    let mut vowel_count: usize = 0;
    let mut has_doubled_letter = false;

    // Iterate through each character in the input string to test if it is "nice"
    // according to the first set of rules, keeping track of a single previous character
    // to allow the rules to be checked in a single iteration.
    let mut last_char = '\0';
    for (i, c) in string.char_indices() {
        if VOWELS.contains(&c) { vowel_count += 1; }

        if i > 0 {
            // If this is not the first character in the string, check whether it matches the
            // previous character. If it does, mark that the string contains a doubled letter.
            if c == last_char { has_doubled_letter = true; }

            // Check whether a string made up of the current character and the last character
            // is in the list of disallowed two character strings and immediately return false
            // if it is.
            let last_string: &str = &vec![last_char, c].iter().collect::<String>();
            if DISALLOWED_SUBSTRINGS.contains(&last_string) { return false; };
        }

        last_char = c;
    }

    // Return true if the string contains at least three vowels and a doubled letter.
    (vowel_count >= 3) && has_doubled_letter
}

// Returns true if the given string is "nice" according to the second set of rules.
fn is_nice_string2(string: &String) -> bool {
    let mut found_matching_pair = false;
    let mut found_repeated_letter = false;
    let mut last_char = '\0';
    let mut last_last_char = '\0';
    for (i, c) in string.char_indices() {
        // Check whether the given string contains two matching, non-overlapping pairs of characters.
        if (i < (string.len() - 2)) && (i > 0) {
            if string[(i + 1)..].find(&vec![last_char, c].iter().collect::<String>()).is_some() {
                found_matching_pair = true;
            }
        }

        // Check whether the given string contains a doubled letter.
        if i > 1 {
            if c == last_last_char {
                found_repeated_letter = true;
            }
        }

        last_last_char = last_char;
        last_char = c;
    }

    return found_matching_pair && found_repeated_letter;
}

fn main() {
    // Part 1: Print the count of "nice" strings in the input list according to the first set of rules.
    println!("{}", input_lines().filter(is_nice_string1).count());

    // Part 2: Print the count of "nice" strings in the input list according to the second set of rules.
    println!("{}", input_lines().filter(is_nice_string2).count());
}
