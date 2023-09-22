use std::collections::VecDeque;

use super::{PrintableError, SyntaxError};

impl PrintableError for SyntaxError {
    fn get_error_str(&self, chars: &Vec<char>) -> String {
        let (line, before, length) = get_line(chars, self.error_start, self.error_end);

        let whitespace = vec![' '; before].iter().collect::<String>();
        let indicator = vec!['^'; length].iter().collect::<String>();

        format!(
            "\n{}\n{}{}\n\n{}{}{}\n{}",
            line, whitespace, indicator, "Syntax error at pos ", self.error_start, ":", self.reason
        )
    }
}

/// Extracts a lin e of code
///
/// - `chars`: Input where to extract the line from
/// - `start_position`: Position where the erroneous code starts
/// - `end_position`: Position where the erroneous code ends
///
/// Returns a tuple of:
///
/// - `String`: The faulty line
/// - `usize`: The amount of chars *before* the faulty code
/// - `usize`: The lenght of the faulty code
///
/// ## Example
///
/// ```
/// let input = String::from("\n\nval number == 50\n\n").chars().into_iter().collect();
/// let start_position = 13;
/// let end_position = 15;
///
/// let (line, before, length) = get_line(&input, start_position, end_position);
///
/// assert_eq!("val number == 50", line);
/// assert_eq!(11, before);
/// assert_eq!(2, length);
/// ```
fn get_line(
    chars: &Vec<char>,
    start_position: usize,
    end_position: usize,
) -> (String, usize, usize) {
    let mut result_chars = VecDeque::<char>::new();

    // Push chars to the front until a new line is found
    let mut before_pos = start_position;
    loop {
        let current_char = chars[before_pos];

        if current_char == '\n' {
            // This is important because before_pos will be used to calculate
            // the number of chars before start_position
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
    let mut after_pos = start_position + 1;
    let char_count = chars.len();
    while after_pos < char_count {
        let current_char = chars[after_pos];

        if current_char == '\n' {
            break;
        }

        result_chars.push_back(current_char);
        after_pos += 1;
    }

    (
        result_chars.iter().collect::<String>(),
        start_position - before_pos,
        end_position - start_position,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        error_handling::{MistiError, PrintableError},
        lexic::get_tokens,
        syntax::construct_ast,
    };

    fn get_error_data(input: String) -> (Vec<char>, MistiError) {
        let tokens = get_tokens(&input).unwrap();
        let error_holder = construct_ast(&tokens);

        match error_holder {
            Ok(_) => panic!(
                "syntax_error test: Input expected to throw error didn't:\n\n{}",
                input
            ),
            Err(error) => {
                let chars: Vec<char> = input.chars().into_iter().collect();

                (chars, error)
            }
        }
    }

    #[test]
    fn should_show_an_error_for_missing_binding_name() {
        let (chars, error) = get_error_data(String::from("val"));
        let actual_err = error.get_error_str(&chars);
        // TODO: Write a better error message (something that explains why it failed)
        let expected_str = format!(
            "\n{}\n{}\n\n{}\n{}",
            "val",
            "^^^",
            "Syntax error at pos 0:",
            "There should be an identifier after a `val` token"
        );

        // assert_eq!(expected_str, actual_err);
    }

    #[test]
    fn should_show_an_error_for_missing_equal_operator() {
        let (chars, error) = get_error_data(String::from("val name"));
        let actual_err = error.get_error_str(&chars);
        // TODO: Write a better error message (something that explains why it failed)
        let expected_str = format!(
            "\n{}\n{}\n\n{}\n{}",
            "val name",
            "    ^^^^",
            "Syntax error at pos 4:",
            "There should be an equal sign `=` after the identifier"
        );

        // assert_eq!(expected_str, actual_err);
    }

    #[test]
    fn should_get_line() {
        let input: Vec<char> = String::from("\n\nval number == 50\n\n")
            .chars()
            .into_iter()
            .collect();

        let start_position = 13;
        let end_position = 15;

        let (line, before, length) = get_line(&input, start_position, end_position);

        assert_eq!("val number == 50", line);
        assert_eq!(11, before);
        assert_eq!(2, length);
    }
}
