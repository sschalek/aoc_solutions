// Advent of Code 2015, Day 8: "Matchsticks"
// https://adventofcode.com/2015/day/8

fn parse_list_string_literal(list_string_literal: &str) -> String {
    #[derive(Copy, Clone)]
    enum ParserMode {
        OutsideString,
        InsideString,
        ProcessingEscape,
        ProcessingAsciiHexEscape,
    }

    let mut parsed_string = String::new();

    let mut mode = ParserMode::OutsideString;
    let mut ascii_code = String::new();
    for c in list_string_literal.chars() {
        match mode {
            ParserMode::OutsideString => {
                match c {
                    '"' => mode = ParserMode::InsideString,
                    _ => panic!(),
                };
            }
            ParserMode::InsideString => {
                match c {
                    '\\' => mode = ParserMode::ProcessingEscape,
                    '"' => mode = ParserMode::OutsideString,
                    _ => parsed_string.push(c),
                };
            }
            ParserMode::ProcessingEscape => {
                match c {
                    '\\' | '"' => {
                        parsed_string.push(c);
                        mode = ParserMode::InsideString;
                    }
                    'x' => {
                        ascii_code = String::new();
                        mode = ParserMode::ProcessingAsciiHexEscape;
                    }
                    _ => panic!(),
                };
            }
            ParserMode::ProcessingAsciiHexEscape => {
                if c.is_ascii_hexdigit() {
                    ascii_code.push(c);
                    if ascii_code.len() == 2 {
                        // let ascii_code_num = u8::from_str_radix(&ascii_code, 16).unwrap();
                        // parsed_string.push(ascii_code_num as char);
                        parsed_string.push('X');
                        mode = ParserMode::InsideString;
                    }
                } else {
                    panic!();
                }
            }
        }
    }
    parsed_string
}

fn encode_list_string(list_string: &str) -> String {
    let mut encoded_string = String::new();

    encoded_string.push('"');
    for c in list_string.chars() {
        if c == '\\' {
            encoded_string.push_str("\\\\");
        } else if c == '"' {
            encoded_string.push_str("\\\"");
        } else {
            encoded_string.push(c);
        }
    }
    encoded_string.push('"');
    encoded_string
}

fn solve(input: &str, _log_fn: Option<fn(&str)>) -> (String, String) {
    // Part 1: Find the total number of characters of code for string literals minus the total number of characters in memory
    // for the values of the strings in the input.
    let mut total_char_diff: isize = 0;
    for l in input.lines() {
        total_char_diff += l.len() as isize;
        let parsed_string = parse_list_string_literal(l);
        total_char_diff -= parsed_string.len() as isize;
    }
    let part1_result = total_char_diff.to_string();

    // Part 2: Find the total number of characters of code for the values of the strings in the input minus the total number of
    // characters in memory for the strings themselves.
    total_char_diff = 0;
    for l in input.lines() {
        let encoded_string = encode_list_string(l);
        total_char_diff += encoded_string.len() as isize;
        total_char_diff -= l.len() as isize;
    }
    let part2_result = total_char_diff.to_string();

    (part1_result, part2_result)
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 8, solve);
