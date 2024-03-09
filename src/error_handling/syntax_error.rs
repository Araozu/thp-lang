use super::utils::{get_line, get_line_number};
use super::{PrintableError, SyntaxError};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error_handling::MistiError, lexic::get_tokens, syntax::construct_ast};

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
