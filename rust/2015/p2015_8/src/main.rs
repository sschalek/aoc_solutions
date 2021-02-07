use std::io::BufRead;

fn input_lines() -> impl Iterator<Item=String> {
    let input_file = std::fs::File::open("input.txt").expect("A file named \"input.txt\" with the problem data must be present in the current directory.");

    // Create a line-based iterator for the file contents.
    let reader = std::io::BufReader::new(input_file);
    reader.lines().map(|l| l.unwrap())
}

fn parse_list_string_literal(list_string_literal: &str) -> String {
    let mut parsed_string = String::new();

    #[derive(Copy, Clone)]
    enum ParserMode {
        OutsideString,
        InsideString,
        ProcessingEscape,
        ProcessingAsciiHexEscape,
    }

    let mut mode = ParserMode::OutsideString;
    let mut ascii_code = String::new();
    for c in list_string_literal.chars() {
        match mode {
            ParserMode::OutsideString => {
                match c {
                    '"' => mode = ParserMode::InsideString,
                    _ => panic!(),
                };
            },
            ParserMode::InsideString => {
                match c {
                    '\\' => mode = ParserMode::ProcessingEscape,
                    '"' => mode = ParserMode::OutsideString,
                    _ => parsed_string.push(c),
                };
            },
            ParserMode::ProcessingEscape => {
                match c {
                    '\\' => {
                        parsed_string.push(c);
                        mode = ParserMode::InsideString;
                    },
                    '"' => {
                        parsed_string.push(c);
                        mode = ParserMode::InsideString;
                    },
                    'x' => {
                        ascii_code = String::new();
                        mode = ParserMode::ProcessingAsciiHexEscape;
                    },
                    _ => panic!(),
                };
            },
            ParserMode::ProcessingAsciiHexEscape => {
                if c.is_ascii_hexdigit() {
                    ascii_code.push(c);
                    if ascii_code.len() == 2 {
                        // let ascii_code_num = u8::from_str_radix(&ascii_code, 16).unwrap();
                        // parsed_string.push(ascii_code_num as char);
                        parsed_string.push('X');
                        mode = ParserMode::InsideString;
                    }
                }
                else {
                    panic!();
                }
            },
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
        }
        else if c == '"' {
            encoded_string.push_str("\\\"");
        }
        else {
            encoded_string.push(c);
        }
    }
    encoded_string.push('"');
    encoded_string
}

fn main() {
    let mut total_char_diff:isize = 0;
    for l in input_lines() {
        total_char_diff += l.len() as isize;
        let parsed_string = parse_list_string_literal(&l);
        total_char_diff -= parsed_string.len() as isize;
    }
    println!("{}", total_char_diff);

    total_char_diff = 0;
    for l in input_lines() {
        let encoded_string = encode_list_string(&l);
        total_char_diff += encoded_string.len() as isize;
        total_char_diff -= l.len() as isize;
    }
    println!("{}", total_char_diff);
}
