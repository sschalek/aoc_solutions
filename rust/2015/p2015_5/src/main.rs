use std::io::BufRead;

fn input_lines() -> impl Iterator<Item=String> {
    let input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");

    // Create a line-based iterator for the file contents.
    let reader = std::io::BufReader::new(input_file);
    return reader.lines().map(|l| l.unwrap());
}

fn is_nice_string1(string: &String) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const DISALLOWED_SUBSTRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

    let mut vowel_count: usize = 0;
    let mut has_doubled_letter = false;

    let mut last_char = '\0';
    for (i, c) in string.char_indices() {
        if VOWELS.contains(&c) { vowel_count += 1; }

        if i > 0 {
            if c == last_char { has_doubled_letter = true; }

            let last_string: &str = &vec![last_char, c].iter().collect::<String>();
            if DISALLOWED_SUBSTRINGS.contains(&last_string) { return false; };
        }

        last_char = c;
    }
    return (vowel_count >= 3) && has_doubled_letter;
}

fn is_nice_string2(string: &String) -> bool {
    let mut found_matching_pair = false;
    let mut last_char = '\0';
    for (i, c) in string.char_indices() {
        if i >= (string.len() - 2) { continue; }

        if i > 0 {
            if string[(i + 1)..].find(&vec![last_char, c].iter().collect::<String>()).is_some() {
                found_matching_pair = true;
                break;
            }
        }

        last_char = c;
    }

    let mut found_repeated_letter = false;
    let mut last_char = '\0';
    let mut last_last_char = '\0';
    for (i, c) in string.char_indices() {
        if i > 1 {
            if c == last_last_char {
                found_repeated_letter = true;
                break;
            }
        }

        last_last_char = last_char;
        last_char = c;
    }

    return found_matching_pair && found_repeated_letter;
}

fn main() {
    println!("{}", input_lines().filter(is_nice_string1).count());
    println!("{}", input_lines().filter(is_nice_string2).count());
}
