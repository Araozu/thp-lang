use serde::Serialize;

use super::utils::{get_line, get_line_number};
use super::PrintableError;

#[derive(Serialize, Debug)]
pub struct SemanticError {
    pub error_start: usize,
    pub error_end: usize,
    pub reason: String,
}

impl PrintableError for SemanticError {
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
