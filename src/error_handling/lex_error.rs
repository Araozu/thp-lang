use std::{collections::VecDeque};
use super::{PrintableError, LexError};

impl PrintableError for LexError {
    // TODO: Count and show line number
    fn get_error_str(&self, chars: &Vec<char>) -> String {

        let (erroneous_code, back_count) = get_line(chars, self.position);

        let mut whitespace = Vec::<char>::new();
        for _ in 0..back_count {
            whitespace.push(' ');
        }
        let whitespace = whitespace.iter().collect::<String>();

        format!(
            "\n{}\n{}^\n\n{}{}\n{}", 
            erroneous_code, 
            whitespace, 
            "Invalid character at pos ",
            self.position + 1,
            self.reason,
        )
    }
}

/// Extracts a line of code from `chars` and the number of characters in the back.
/// `pos` indicates a position, from where to extract the line.
/// 
/// Ex. Given:
/// - `input = "first line\nsecond line\nthird line"`
/// - `pos = 15`
/// 
/// this function should return `("second line", 4)`
fn get_line(chars: &Vec<char>, pos: usize) -> (String, usize) {
    let mut result_chars = VecDeque::<char>::new();

    // Push chars to the front until a new line is found
    // TODO: refactor
    let mut before_pos = pos;
    loop {
        let current_char = chars[before_pos];

        if current_char == '\n' {
            before_pos += 1;
            break;
        }

        result_chars.push_front(current_char);

        if before_pos == 0 {
            break;
        }

        before_pos -= 1;
    }

    // Push chars to the end until a new line is found
    let mut after_pos = pos + 1;
    let char_count = chars.len();
    while after_pos < char_count {
        let current_char = chars[after_pos];

        if current_char == '\n' {
            break;
        }

        result_chars.push_back(current_char);
        after_pos += 1;
    }

    (result_chars.iter().collect::<String>(), pos - before_pos)
}



#[cfg(test)]
mod tests {
    use crate::lexic;
    use super::*;

    #[test]
    fn test_error_msg() {
        let input = String::from("val name' = 20");
        let result = lexic::get_tokens(&input);

        match result {
            Ok(_) => assert!(false),
            Err(err_data) => {
                let chars: Vec<char> = input.chars().into_iter().collect();
                let err_str = err_data.get_error_str(&chars);

                // TODO: check for line number
                let expected_str = format!(
                    "\n{}\n{}^\n\nInvalid character at pos 9",
                    "val name' = 20",
                    "        "
                );

                assert_eq!(
                    expected_str,
                    err_str,
                );
            }
        }
    }

    #[test]
    fn should_extract_line() {
        let input = String::from("first line\nsecond line\nthird line");
        let chars: Vec<char> = input.chars().into_iter().collect();

        let (result, back_count) = get_line(&chars, 15);

        assert_eq!("second line", result);
        assert_eq!(4, back_count);


        let input = String::from("val binding = 322");
        let chars: Vec<char> = input.chars().into_iter().collect();

        let (result, back_count) = get_line(&chars, 6);

        assert_eq!("val binding = 322", result);
        assert_eq!(6, back_count);
    }
}
