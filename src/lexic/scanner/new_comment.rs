use super::token::Token;
use crate::{
    error_handling::LexError,
    lexic::{utils, LexResult},
};

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

/// Scans a multiline commend
/// This function assumes that the character at `start_pos` is '/'
/// and the character at `start_pos + 1` is '*'
pub fn scan_multiline(chars: &Vec<char>, start_pos: usize) -> LexResult {
    match multiline_impl(chars, start_pos + 2) {
        Some((value, next_position)) => LexResult::Some(
            Token::new_multiline_comment(value.iter().collect(), start_pos),
            next_position,
        ),
        None => {
            // Throw an error: Incomplete multiline comment
            LexResult::Err(LexError {
                position: start_pos,
                // TODO: add an end_position
                reason: "Unfinished multiline commend".into(),
            })
        }
    }
}

fn multiline_impl(chars: &Vec<char>, start_pos: usize) -> Option<(Vec<char>, usize)> {
    let mut current_position = start_pos;
    let mut result = Vec::<char>::new();

    loop {
        match chars.get(current_position) {
            Some('/') => {
                match chars.get(current_position + 1) {
                    Some('*') => {
                        // Scan nested comment
                        let (mut nested, next_position) =
                            match multiline_impl(chars, current_position + 2) {
                                Some(v) => v,
                                None => {
                                    // The nested comment is not closed.
                                    return None;
                                }
                            };
                        result.push('/');
                        result.push('*');
                        result.append(&mut nested);
                        result.push('*');
                        result.push('/');
                        current_position = next_position;
                    }
                    Some(c) => {
                        // Append both characters
                        result.push('/');
                        result.push(*c);
                    }
                    None => return None,
                }
            }
            Some('*') => {
                // Check for the end of a comment
                match chars.get(current_position + 1) {
                    Some('/') => {
                        // Create and return the token,
                        // ignoring the `*/`
                        return Some((result, current_position + 2));
                    }
                    Some(c) => {
                        // Append both and continue
                        result.push('*');
                        result.push(*c);
                        current_position += 2;
                    }
                    None => {
                        // Throw an error
                        return None;
                    }
                }
            }
            Some(c) => {
                // Append and continue
                result.push(*c);
                current_position += 1;
            }
            None => {
                // TODO: Also return the position where this token ends,
                // to display better error messages.
                // Requires LexError to implement an end_position field
                return None;
            }
        }
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

    #[test]
    fn should_scan_multiline() {
        let input = str_to_vec("/**/");

        let result = scan_multiline(&input, 0);
        match result {
            LexResult::Some(t, next) => {
                assert_eq!(4, next);
                assert_eq!("", t.value);
                assert_eq!(0, t.position);
                assert_eq!(TokenType::MultilineComment, t.token_type);
            }
            _ => {
                panic!("Expected a multine comment")
            }
        }
    }

    #[test]
    fn should_scan_multiline_2() {
        let input = str_to_vec("/* my comment */");

        let result = scan_multiline(&input, 0);
        match result {
            LexResult::Some(t, next) => {
                assert_eq!(16, next);
                assert_eq!(" my comment ", t.value);
                assert_eq!(0, t.position);
                assert_eq!(TokenType::MultilineComment, t.token_type);
            }
            _ => {
                panic!("Expected a multine comment")
            }
        }
    }

    #[test]
    fn should_scan_multiline_with_multiple_lines() {
        let input = str_to_vec("/* my\ncomment */");

        let result = scan_multiline(&input, 0);
        match result {
            LexResult::Some(t, next) => {
                assert_eq!(16, next);
                assert_eq!(" my\ncomment ", t.value);
                assert_eq!(0, t.position);
                assert_eq!(TokenType::MultilineComment, t.token_type);
            }
            _ => {
                panic!("Expected a multine comment")
            }
        }
    }

    #[test]
    fn should_not_scan_multiline_comment_if_invalid() {
        let input = str_to_vec("/* my\ncomment");

        let result = scan_multiline(&input, 0);
        match result {
            LexResult::Err(error) => {
                assert_eq!(0, error.position)
            }
            _ => {
                panic!("Expected an error scannning an incomplete multiline comment")
            }
        }
    }

    #[test]
    fn should_scan_multiline_comments_with_asterisk() {
        let input = str_to_vec("/* my * comment */");

        let result = scan_multiline(&input, 0);
        match result {
            LexResult::Some(t, next) => {
                assert_eq!(18, next);
                assert_eq!(" my * comment ", t.value);
                assert_eq!(0, t.position);
                assert_eq!(TokenType::MultilineComment, t.token_type);
            }
            _ => {
                panic!("Expected a multine comment")
            }
        }
    }

    #[test]
    fn shoud_scan_nested_multiline_comments() {
        let input = str_to_vec("/* my /* comment */ */");

        let result = scan_multiline(&input, 0);
        match result {
            LexResult::Some(t, next) => {
                assert_eq!(22, next);
                assert_eq!(" my /* comment */ ", t.value);
                assert_eq!(0, t.position);
                assert_eq!(TokenType::MultilineComment, t.token_type);
            }
            _ => {
                panic!("Expected a multine comment")
            }
        }
    }
}
