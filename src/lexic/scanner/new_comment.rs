use super::token::Token;
use crate::lexic::{utils, LexResult};

/// Scans a new line.
///
/// Assummes that `start_pos` and `start_pos + 1` point to a slash `/`
///
/// This methods always succeedes
pub fn scan(chars: &Vec<char>, start_pos: usize) -> LexResult {
    let (comment_content, next_pos) =
        scan_any_except_new_line(chars, start_pos + 2, String::from(""));
    let token = Token::new_comment(format!("//{}", comment_content), start_pos);

    LexResult::Some(token, next_pos)
}

fn scan_any_except_new_line(
    chars: &Vec<char>,
    start_pos: usize,
    current: String,
) -> (String, usize) {
    match chars.get(start_pos) {
        Some(c) if *c == '\n' => (current, start_pos),
        Some(c) => scan_any_except_new_line(chars, start_pos + 1, utils::str_append(current, *c)),
        None => (current, start_pos),
    }
}

#[cfg(test)]
mod tests {
    use crate::lexic::scanner::TokenType;

    use super::*;

    fn str_to_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn should_scan_empty_comment() {
        let input = str_to_vec("//");
        let start_pos = 0;

        let result = scan(&input, start_pos);
        match result {
            LexResult::Some(t, next) => {
                assert_eq!(2, next);
                assert_eq!("//", t.value);
                assert_eq!(0, t.position);
                assert_eq!(TokenType::Comment, t.token_type);
            }
            _ => {
                panic!()
            }
        }
    }

    #[test]
    fn should_scan_until_new_line() {
        let input = str_to_vec("  // some comment\n// other comment");
        let start_pos = 2;

        let result = scan(&input, start_pos);
        match result {
            LexResult::Some(t, next) => {
                assert_eq!(17, next);
                assert_eq!("// some comment", t.value);
                assert_eq!(start_pos, t.position);
                assert_eq!(TokenType::Comment, t.token_type);
            }
            _ => {
                panic!()
            }
        }
    }
}
