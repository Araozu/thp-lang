use crate::lexic::{
    token,
    utils, LexResult,
};
use crate::error_handling::LexError;

/// Function to scan a string
///
/// This function assumes that `start_pos` is after the first double quote,
/// e.g. if the input is `"hello"`, `start_pos == 1`
pub fn scan(chars: &Vec<char>, start_pos: usize) -> LexResult {
    scan_impl(chars, start_pos, String::from(""))
}

/// Recursive function that does the scanning
pub fn scan_impl(chars: &Vec<char>, start_pos: usize, current: String) -> LexResult {
    match chars.get(start_pos) {
        Some(c) if *c == '"' => {
            LexResult::Some(token::new_string(current, start_pos as i32), start_pos + 1)
        }
        Some(c) if *c == '\n' => {
            LexResult::Err(LexError {
                position: start_pos,
                reason: String::from("Unexpected new line inside a string.")
            })
        }
        Some(c) if *c == '\\' => {
            if let Some(escape) = test_escape_char(chars, start_pos + 1) {
                scan_impl(
                    chars,
                    start_pos + 2, 
                    utils::str_append(current, escape),
                )
            }
            else {
                // Ignore the backslash
                scan_impl(
                    chars,
                    start_pos + 1,
                    current,
                )
            }
        }
        Some(c) => {
            scan_impl(
                chars,
                start_pos + 1,
                utils::str_append(current, *c),
            )
        }
        None => {
            LexResult::Err(LexError {
                position: start_pos,
                reason: String::from("Incomplete string found")
            })
        }
    }
}


/// Checks if the char at `start_pos` is a escape character
fn test_escape_char(chars: &Vec<char>, start_pos: usize) -> Option<char> {
    if let Some(c) = chars.get(start_pos) {
        match *c {
            'n' => Some('\n'),
            '"' => Some('"'),
            'r' => Some('\r'),
            '\\' => Some('\\'),
            't' => Some('\t'),
            _ => None,
        }
    }
    else {
        None
    }
}




#[cfg(test)]
mod tests {
    use crate::lexic::token::TokenType;

    use super::*;

    fn str_to_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn should_scan_an_empty_string() {
        let input = str_to_vec("\"\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(2, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("", token.value);
        }
        else {panic!()}
    }

    #[test]
    fn should_scan_a_string_with_contents() {
        let input = str_to_vec("\"Hello, world!\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(15, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Hello, world!", token.value);
        }
        else {panic!()}
    }

    #[test]
    fn should_not_scan_a_new_line() {
        let input = str_to_vec("\"Hello,\nworld!\"");
        let start_pos = 1;
        if let LexResult::Err(reason) = scan(&input, start_pos) {
            assert_eq!("Unexpected new line inside a string.", reason.reason)
        }
        else {panic!()}
    }

    #[test]
    fn should_scan_escape_characters() {
        let input = str_to_vec("\"Sample\\ntext\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\ntext", token.value);
        }
        else {panic!()}

        let input = str_to_vec("\"Sample\\\"text\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\"text", token.value);
        }
        else {panic!()}

        let input = str_to_vec("\"Sample\\rtext\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\rtext", token.value);
        }
        else {panic!()}

        let input = str_to_vec("\"Sample\\\\text\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\\text", token.value);
        }
        else {panic!()}

        let input = str_to_vec("\"Sample\\ttext\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\ttext", token.value);
        }
        else {panic!()}

        let input = str_to_vec("\"Sample\\ text\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample text", token.value);
        }
        else {panic!()}
    }
}
