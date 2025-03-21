use crate::error_handling::error_messages::LEX_INCOMPLETE_STRING;
use crate::error_handling::{ErrorContainer, ErrorLabel};
use crate::lexic::token::Token;
use crate::lexic::{utils, LexResult};

/// Function to scan a string
///
/// This function assumes that `start_pos` is after the first double quote,
/// e.g. if the input is `"hello"`, `start_pos == 1`
pub fn scan(chars: &Vec<char>, start_pos: usize) -> LexResult {
    scan_impl(chars, start_pos, String::from(""))
}

// TODO: This can be iterative instead of recursive

/// Recursive function that does the scanning
pub fn scan_impl(chars: &Vec<char>, start_pos: usize, current: String) -> LexResult {
    match chars.get(start_pos) {
        Some(c) if *c == '"' => {
            // start_pos is the position where the token ENDS, not where it STARTS,
            // so this is used to retrieve the original START position of the token
            // 1 is added to account for the opening `"`
            let current_len = current.len() + 1;

            LexResult::Some(
                Token::new_string(current, start_pos - current_len),
                start_pos + 1,
            )
        }
        Some(c) if *c == '\n' => {
            let string_start_pos = start_pos - (current.len() + 1);
            let label_2 = ErrorLabel {
                message: String::from("The line ends here"),
                start: start_pos,
                end: start_pos + 1,
            };
            let label_1 = ErrorLabel {
                message: String::from("The string starts here"),
                start: string_start_pos,
                end: string_start_pos + 1,
            };
            let econtainer = ErrorContainer {
                error_code: LEX_INCOMPLETE_STRING,
                error_offset: start_pos,
                labels: vec![label_1, label_2],
                note: Some(String::from("Strings cannot have newlines")),
                help: None,
            };
            LexResult::Err(econtainer)
        }
        Some(c) if *c == '\\' => {
            if let Some(escape) = test_escape_char(chars, start_pos + 1) {
                // This should only detect an escaped `"`
                scan_impl(chars, start_pos + 2, format!("{}{}", current, escape))
            } else {
                scan_impl(chars, start_pos + 1, utils::str_append(current, *c))
            }
        }
        Some(c) => scan_impl(chars, start_pos + 1, utils::str_append(current, *c)),
        None => {
            let string_start_pos = start_pos - (current.len() + 1);
            let label_1 = ErrorLabel {
                message: String::from("The string starts here"),
                start: string_start_pos,
                end: string_start_pos + 1,
            };
            let label_2 = ErrorLabel {
                message: String::from("The code ends here"),
                start: start_pos,
                end: start_pos + 1,
            };
            let econtainer = ErrorContainer {
                error_code: LEX_INCOMPLETE_STRING,
                error_offset: start_pos,
                labels: vec![label_1, label_2],
                note: None,
                help: None,
            };

            LexResult::Err(econtainer)
        }
    }
}

/// Checks if the char at `start_pos` is a escape character
fn test_escape_char(chars: &Vec<char>, start_pos: usize) -> Option<String> {
    if let Some(c) = chars.get(start_pos) {
        match *c {
            // Escape sequences ignored: They are passed as is to JS
            'n' => Some(String::from("\\n")),
            '"' => Some(String::from("\\\"")),
            'r' => Some(String::from("\\r")),
            '\\' => Some(String::from("\\\\")),
            't' => Some(String::from("\\t")),
            _ => None,
        }
    } else {
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
            assert_eq!(0, token.position);
        } else {
            panic!()
        }
    }

    #[test]
    fn should_scan_a_string_with_contents() {
        let input = str_to_vec("\"Hello, world!\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(15, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Hello, world!", token.value);
            assert_eq!(0, token.position);
        } else {
            panic!()
        }
    }

    #[test]
    fn should_not_scan_a_new_line() {
        let input = str_to_vec("\"Hello,\nworld!\"");
        let start_pos = 1;
        if let LexResult::Err(err) = scan(&input, start_pos) {
            assert_eq!(LEX_INCOMPLETE_STRING, err.error_code)
        } else {
            panic!()
        }
    }

    #[test]
    fn should_scan_escape_characters() {
        let input = str_to_vec("\"Sample\\ntext\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\\ntext", token.value);
            assert_eq!(0, token.position);
        } else {
            panic!()
        }

        let input = str_to_vec("\"Sample\\\"text\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\\\"text", token.value);
            assert_eq!(0, token.position);
        } else {
            panic!()
        }

        let input = str_to_vec("\"Sample\\rtext\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\\rtext", token.value);
            assert_eq!(0, token.position);
        } else {
            panic!()
        }

        let input = str_to_vec("\"Sample\\\\text\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\\\\text", token.value);
            assert_eq!(0, token.position);
        } else {
            panic!()
        }

        let input = str_to_vec("\"Sample\\ttext\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\\ttext", token.value);
            assert_eq!(0, token.position);
        } else {
            panic!()
        }

        let input = str_to_vec("\"Sample\\ text\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\\ text", token.value);
            assert_eq!(0, token.position);
        } else {
            panic!()
        }
    }

    #[test]
    fn should_scan_non_escape_characters_preceded_by_bsls() {
        let input = str_to_vec("\"Sample\\atext\"");
        let start_pos = 1;
        if let LexResult::Some(token, next) = scan(&input, start_pos) {
            assert_eq!(14, next);
            assert_eq!(TokenType::String, token.token_type);
            assert_eq!("Sample\\atext", token.value);
            assert_eq!(0, token.position);
        } else {
            panic!()
        }
    }

    #[test]
    fn shouldnt_panic_when_encountering_eof_after_bsls() {
        let input = str_to_vec("\"Sample\\");
        let start_pos = 1;
        let result = scan(&input, start_pos);

        match result {
            LexResult::Err(err) => {
                assert_eq!(LEX_INCOMPLETE_STRING, err.error_code)
            }
            _ => panic!("expected an error"),
        }
    }

    #[test]
    fn should_not_scan_an_unfinished_string() {
        let input = str_to_vec("\"Hello, world!");
        let result = scan(&input, 1);

        match result {
            LexResult::Err(err) => {
                assert_eq!(LEX_INCOMPLETE_STRING, err.error_code)
            }
            _ => panic!("expected an error"),
        }
    }
}
