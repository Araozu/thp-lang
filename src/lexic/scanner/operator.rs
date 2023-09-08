use crate::lexic::{token::Token, utils, LexResult};

/// Function to scan an operator
///
/// This function assumes the character at `start_pos` is an operator
pub fn scan(chars: &Vec<char>, start_pos: usize) -> LexResult {
    scan_impl(chars, start_pos, String::from(""))
}

pub fn scan_impl(chars: &Vec<char>, start_pos: usize, current: String) -> LexResult {
    match chars.get(start_pos) {
        Some(c) if utils::is_operator(*c) => {
            scan_impl(chars, start_pos + 1, utils::str_append(current, *c))
        }
        _ => {
            // start_pos is the position where the token ENDS, not where it STARTS,
            // so this is used to retrieve the original START position of the token
            let current_len = current.len();

            LexResult::Some(
                Token::new_operator(current, start_pos - current_len),
                start_pos,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::token::TokenType;

    fn str_to_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    // Should scan operators of length 1
    #[test]
    fn test_1() {
        let operators = vec![
            "+", "-", "=", "*", "!", "\\", "/", "|", "@", "#", "$", "~", "%", "&", "?", "<", ">",
            "^", ".", ":",
        ];

        for op in operators {
            let input = str_to_vec(op);
            let start_pos = 0;
            match scan(&input, start_pos) {
                LexResult::Some(token, next) => {
                    assert_eq!(1, next);
                    assert_eq!(TokenType::Operator, token.token_type);
                    assert_eq!(op, token.value);
                    assert_eq!(0, token.position);
                }
                _ => panic!(),
            }
        }
    }

    // Should scan operators of length 2
    #[test]
    fn test_2() {
        let operators = vec![
            "<<", ">>", "<|", "|>", "+>", "<+", "+=", "-=", "?.", "??", "?:", "*=", "/=", "==",
            "!=",
        ];

        for op in operators {
            let input = str_to_vec(op);
            let start_pos = 0;
            match scan(&input, start_pos) {
                LexResult::Some(token, next) => {
                    assert_eq!(2, next);
                    assert_eq!(TokenType::Operator, token.token_type);
                    assert_eq!(op, token.value);
                    assert_eq!(0, token.position);
                }
                _ => panic!(),
            }
        }
    }
}
