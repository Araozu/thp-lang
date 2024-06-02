use super::token::TokenType;
use crate::lexic::{token::Token, LexResult};

/// Function to handle new lines
///
/// Assumes the char at start_pos is a new line
pub fn scan(chars: &Vec<char>, start_pos: usize) -> LexResult {
    let current = chars.get(start_pos);

    match current {
        Some(c) if *c == '\n' => scan(chars, start_pos + 1),
        Some(c) if *c == ' ' => match look_ahead_for_new_line(chars, start_pos + 1) {
            Some(next_pos) => scan(chars, next_pos),
            None => {
                let token = Token::new(String::from(""), start_pos, TokenType::NewLine);
                LexResult::Some(token, start_pos)
            }
        },
        None => {
            // Here EOF is found. Don't emit a newline, but consume the tokens.
            // To consume the tokens without returning a newline, we use LexResult::Multiple
            LexResult::Multiple(vec![], start_pos)
        }
        _ => {
            let token = Token::new(String::from(""), start_pos, TokenType::NewLine);
            LexResult::Some(token, start_pos)
        }
    }
}

/// Returns the position after the new line
fn look_ahead_for_new_line(chars: &Vec<char>, pos: usize) -> Option<usize> {
    match chars.get(pos) {
        Some(c) if *c == ' ' => look_ahead_for_new_line(chars, pos + 1),
        Some(c) if *c == '\n' => Some(pos + 1),
        Some(_) | None => None,
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
    fn should_emit_newline() {
        let input = str_to_vec("\n_");
        let start_pos = 0;

        if let LexResult::Some(token, next_pos) = scan(&input, start_pos) {
            assert_eq!(TokenType::NewLine, token.token_type);
            assert_eq!(1, next_pos);
        } else {
            panic!()
        }
    }

    #[test]
    fn should_emit_a_single_newline_with_multiple_new_lines() {
        let input = str_to_vec("\n\n\n_");
        let start_pos = 0;

        if let LexResult::Some(token, next_pos) = scan(&input, start_pos) {
            assert_eq!(TokenType::NewLine, token.token_type);
            assert_eq!(3, next_pos);
        } else {
            panic!()
        }

        let input = str_to_vec("\n\n\naToken");
        let start_pos = 0;

        if let LexResult::Some(token, next_pos) = scan(&input, start_pos) {
            assert_eq!(TokenType::NewLine, token.token_type);
            assert_eq!(3, next_pos);
        } else {
            panic!()
        }
    }

    #[test]
    fn should_emit_a_single_semicolon_with_multiple_new_lines_and_whitespace() {
        let input = str_to_vec("\n \n  \n_");
        let start_pos = 0;

        if let LexResult::Some(token, next_pos) = scan(&input, start_pos) {
            assert_eq!(TokenType::NewLine, token.token_type);
            assert_eq!(6, next_pos);
        } else {
            panic!()
        }

        let input = str_to_vec("\n \n  \n    aToken");
        let start_pos = 0;

        if let LexResult::Some(token, next_pos) = scan(&input, start_pos) {
            assert_eq!(TokenType::NewLine, token.token_type);
            assert_eq!(6, next_pos);
        } else {
            panic!()
        }

        let input = str_to_vec("\n \n  \n    _");
        let start_pos = 0;

        if let LexResult::Some(token, next_pos) = scan(&input, start_pos) {
            assert_eq!(TokenType::NewLine, token.token_type);
            assert_eq!(6, next_pos);
        } else {
            panic!()
        }
    }

    #[test]
    fn shouldnt_emit_newline_if_eof_is_found() {
        let input = str_to_vec("\n\n");

        match scan(&input, 0) {
            LexResult::Multiple(vec, next_pos) => {
                assert_eq!(vec.len(), 0);
                assert_eq!(next_pos, 2);
            }
            _ => {
                panic!("Expected a multiple result")
            }
        }
    }
}
