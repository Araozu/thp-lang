use super::{LexError, PrintableError};
use std::collections::VecDeque;

impl PrintableError for LexError {
    // TODO: Count and show line number
    fn get_error_str(&self, chars: &Vec<char>) -> String {
        let line_number = get_line_number(chars, self.position);
        let (erroneous_code, back_count) = get_line(chars, self.position);

        let whitespace = " ".repeat(back_count + line_number.to_string().len() + 1);

        format!(
            "\n{}|{}\n{}^\n\n{}{}\n{}",
            line_number,
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
            // This is important because before_pos will be used to calculate
            // the number of chars before pos
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

fn get_line_number(chars: &Vec<char>, target_pos: usize) -> usize {
    let mut count = 1;

    for (pos, char) in chars.iter().enumerate() {
        if pos >= target_pos {
            break;
        }

        if *char == '\n' {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic;

    #[test]
    fn test_error_msg() {
        let input = String::from("val name' = 20");
        let result = lexic::get_tokens(&input);

        match result {
            Ok(_) => assert!(false),
            Err(err_data) => {
                let chars: Vec<char> = input.chars().into_iter().collect();
                let err_str = err_data.get_error_str(&chars);

                let expected_str = format!(
                    "\n1|{}\n  {}^\n\nInvalid character at pos 9\n{}",
                    "val name' = 20", "        ", "Unrecognized character `'` (escaped: `\\'`)"
                );

                assert_eq!(expected_str, err_str,);
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

    #[test]
    fn should_get_line_number() {
        let input = String::from("one\ntwo\nthree\nfour\nfive\nsix\nseven\neight\nnine\nten");
        let chars: Vec<char> = input.chars().into_iter().collect();

        let line_number = get_line_number(&chars, 11);
        assert_eq!(3, line_number);

        let line_number = get_line_number(&chars, 0);
        assert_eq!(1, line_number);

        let line_number = get_line_number(&chars, 3);
        assert_eq!(1, line_number);

        let line_number = get_line_number(&chars, 15);
        assert_eq!(4, line_number);
    }
}
