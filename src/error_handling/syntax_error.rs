use super::{PrintableError, SyntaxError};
use std::collections::VecDeque;

impl PrintableError for SyntaxError {
    fn get_error_str(&self, chars: &Vec<char>) -> String {
        let (line, before, length) = get_line(chars, self.error_start, self.error_end);

        let line_number = get_line_number(chars, self.error_start);
        let line_number_whitespace = " ".repeat(line_number.to_string().len());

        let whitespace = vec![' '; before].iter().collect::<String>();
        let indicator = vec!['^'; length].iter().collect::<String>();
        let reason = &self.reason;

        format!(
            r#"
{line_number_whitespace} |
{line_number           } | {line}
{line_number_whitespace} | {whitespace}{indicator}

{reason} at line {line_number}:{before}"#,
        )
    }
}

/// Extracts a line of code
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
    use crate::{
        error_handling::MistiError,
        lexic::get_tokens,
        syntax::construct_ast,
    };

    fn _get_error_data(input: String) -> (Vec<char>, MistiError) {
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
