use super::token::TokenType;
use crate::lexic::{token::Token, utils, LexResult};

/// Checks if a String is a keyword, and returns its TokenType
fn str_is_keyword(s: &String) -> Option<TokenType> {
    match s.as_str() {
        "var" => Some(TokenType::VAR),
        "val" => Some(TokenType::VAL),
        "fun" => Some(TokenType::FUN),
        _ => None,
    }
}

/// Scans an identifier. This function assumes that `start_pos` is the start of
/// a valid identifier
pub fn scan(start_char: char, chars: &Vec<char>, start_pos: usize) -> LexResult {
    // The scanning is done by this recursive function
    scan_impl(
        chars,
        start_pos + 1,
        format!("{}", start_char),
        utils::is_uppercase(start_char),
    )
}

/// Recursive funtion that scans the identifier
fn scan_impl(chars: &Vec<char>, start_pos: usize, current: String, is_datatype: bool) -> LexResult {
    match chars.get(start_pos) {
        Some(c) if utils::is_identifier_char(*c) => scan_impl(
            chars,
            start_pos + 1,
            utils::str_append(current, *c),
            is_datatype,
        ),
        _ => {
            // start_pos is the position where the token ENDS, not where it STARTS,
            // so this is used to retrieve the original START position of the token
            let current_len = current.len();
            if let Some(token_type) = str_is_keyword(&current) {
                LexResult::Some(
                    Token::new(current, start_pos - current_len, token_type),
                    start_pos,
                )
            } else if is_datatype {
                LexResult::Some(
                    Token::new_datatype(current, start_pos - current_len),
                    start_pos,
                )
            } else {
                LexResult::Some(
                    Token::new_identifier(current, start_pos - current_len),
                    start_pos,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexic::token::TokenType;

    use super::*;

    fn str_to_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    // Should scan a lenght 1 identifier
    #[test]
    fn test_1() {
        let input = str_to_vec("_");
        let start_pos = 0;
        match scan(*input.get(0).unwrap(), &input, start_pos) {
            LexResult::Some(token, next) => {
                assert_eq!(1, next);
                assert_eq!(TokenType::Identifier, token.token_type);
                assert_eq!("_", token.value);
            }
            _ => panic!(),
        }

        let input = str_to_vec("i");
        let start_pos = 0;
        match scan(*input.get(0).unwrap(), &input, start_pos) {
            LexResult::Some(token, next) => {
                assert_eq!(1, next);
                assert_eq!(TokenType::Identifier, token.token_type);
                assert_eq!("i", token.value);
            }
            _ => panic!(),
        }
    }

    // Should scan a lenght 2 identifier
    #[test]
    fn test_2() {
        let operators = vec![
            "_a", "_z", "_A", "_Z", "__", "_0", "_9", "aa", "az", "aA", "aZ", "a_", "a0", "a9",
            "za", "zz", "zA", "zZ", "z_", "z0", "z9",
        ];

        for op in operators {
            let input = str_to_vec(op);
            let start_pos = 0;
            match scan(*input.get(0).unwrap(), &input, start_pos) {
                LexResult::Some(token, next) => {
                    assert_eq!(2, next);
                    assert_eq!(TokenType::Identifier, token.token_type);
                    assert_eq!(op, token.value);
                }
                _ => panic!(),
            }
        }
    }

    // Should scan long identifiers
    #[test]
    fn test_3() {
        let operators = vec![
            "_validIdentifier",
            "iterationCount",
            "buffer",
            "aVeryLongIdentifier2WithSome5Numbers67InBetween1",
        ];

        for op in operators {
            let input = str_to_vec(op);
            let start_pos = 0;
            match scan(*input.get(0).unwrap(), &input, start_pos) {
                LexResult::Some(token, next) => {
                    assert_eq!(input.len(), next);
                    assert_eq!(TokenType::Identifier, token.token_type);
                    assert_eq!(op, token.value);
                }
                _ => panic!(),
            }
        }
    }

    // Should scan keywords
    #[test]
    fn test_4() {
        let input = str_to_vec("var");
        let start_pos = 0;
        if let LexResult::Some(token, next) = scan(*input.get(0).unwrap(), &input, start_pos) {
            assert_eq!(3, next);
            assert_eq!(TokenType::VAR, token.token_type);
            assert_eq!("var", token.value);
            assert_eq!(0, token.position);
        } else {
            panic!()
        }

        let input = str_to_vec("val");
        let start_pos = 0;
        if let LexResult::Some(token, next) = scan(*input.get(0).unwrap(), &input, start_pos) {
            assert_eq!(3, next);
            assert_eq!(TokenType::VAL, token.token_type);
            assert_eq!("val", token.value);
            assert_eq!(0, token.position);
        } else {
            panic!()
        }
    }
}
