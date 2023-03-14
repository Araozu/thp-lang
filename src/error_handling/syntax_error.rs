use super::{PrintableError, SyntaxError};

impl PrintableError for SyntaxError {
    fn get_error_str(&self, chars: &Vec<char>) -> String {
        String::from("Syntax error: NOT IMPLEMENTED")
    }
}
