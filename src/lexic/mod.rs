mod scanner;
mod utils;

pub mod token;

use crate::error_handling::{LexError, MistiError};
use token::Token;

use self::token::TokenType;

type Chars = Vec<char>;

/// Represents the result of scanning a single token from the input
pub enum LexResult {
    /// A token was found. The first element is the token, and the
    /// second element is the position in the input after the token.
    ///
    /// E.g., given an input
    ///
    /// "`identifier 55`"
    ///
    /// scanning from a position `0`, the result would be
    ///
    /// `Some(Token("identifier"), 10)`.
    ///
    /// where:
    /// - `Token("identifier")` is the token
    /// - `10` is the position where the token ends, and from where the next token
    /// should be scanned
    Some(Token, usize),
    /// Multiple tokens
    Multiple(Vec<Token>, usize),
    /// No token was found. This indicates that EOF has been reached.
    ///
    /// Contains the last position, which should be the input lenght - 1
    None(usize),
    /// An error was found while scanning.
    Err(LexError),
}

/// Scans and returns all the tokens in the input String
pub fn get_tokens(input: &String) -> Result<Vec<Token>, MistiError> {
    let chars: Vec<char> = input.chars().into_iter().collect();
    let mut results = Vec::new();
    let mut current_pos: usize = 0;
    let mut indentation_stack = vec![0];
    // Used to emit INDENT & DEDENT tokens
    let mut at_new_line = false;

    while has_input(&chars, current_pos) {
        match next_token(&chars, current_pos, &mut indentation_stack, at_new_line) {
            LexResult::Some(token, next_pos) => {
                at_new_line = token.token_type == TokenType::NewLine;

                results.push(token);
                current_pos = next_pos;
            }
            LexResult::Multiple(tokens, next_pos) => {
                at_new_line = tokens.last().unwrap().token_type == TokenType::NewLine;

                results.extend(tokens);
                current_pos = next_pos;
            }
            LexResult::None(next_pos) => {
                current_pos = next_pos;
            }
            LexResult::Err(error_info) => {
                return Err(MistiError::Lex(error_info));
            }
        }
    }

    // emit DEDENT tokens for each entry left in the indentation_stack,
    // except the first one (which is 0)
    for _ in 0..indentation_stack.len() - 1 {
        results.push(Token::new_dedent(current_pos));
    }

    // Push EOF
    results.push(Token::new_eof(0));
    Ok(results)
}

/// Scans a single token from `chars`, starting from `current_pos`
fn next_token(
    chars: &Chars,
    current_pos: usize,
    indentation_stack: &mut Vec<usize>,
    at_new_line: bool,
) -> LexResult {
    let mut current_pos = current_pos;

    if at_new_line {
        return handle_indentation(chars, current_pos, indentation_stack);
    } else if !at_new_line && peek(chars, current_pos) == ' ' {
        // Consume whitespace
        current_pos += 1;
        while peek(chars, current_pos) == ' ' {
            current_pos += 1;
        }
    }

    // If EOF is reached return only the current position
    if peek(chars, current_pos) == '\0' {
        return LexResult::None(current_pos);
    }

    let next_char = peek(chars, current_pos);

    // Scanners
    None.or_else(|| scanner::number(next_char, chars, current_pos))
        .or_else(|| scanner::identifier(next_char, chars, current_pos))
        .or_else(|| scanner::datatype(next_char, chars, current_pos))
        .or_else(|| scanner::string(next_char, chars, current_pos))
        .or_else(|| scanner::new_comment(next_char, chars, current_pos))
        .or_else(|| scanner::operator(next_char, chars, current_pos))
        .or_else(|| scanner::grouping_sign(next_char, chars, current_pos))
        .or_else(|| scanner::new_line(next_char, chars, current_pos))
        .unwrap_or_else(|| {
            let error = LexError {
                position: current_pos,
                reason: format!(
                    "Unrecognized character `{}` (escaped: `{}`)",
                    next_char,
                    next_char.escape_default().to_string(),
                ),
            };
            LexResult::Err(error)
        })
}

fn handle_indentation(
    chars: &Chars,
    current_pos: usize,
    indentation_stack: &mut Vec<usize>,
) -> LexResult {
    // Count the number of spaces
    let mut spaces = 0;
    let mut sub_pos = current_pos;
    while peek(chars, sub_pos) == ' ' {
        spaces += 1;
        sub_pos += 1;
    }

    // Compare the number of spaces with the top of the stack
    let top = indentation_stack.last().unwrap_or(&0);

    if spaces > *top {
        // Push the new indentation level
        indentation_stack.push(spaces);
        return LexResult::Some(Token::new_indent(current_pos), current_pos + spaces);
    } else if spaces < *top {
        // Emit a DEDENT token for each indentation level that is decreased
        let mut dedent_tokens = Vec::<Token>::new();

        while let Some(new_top) = indentation_stack.last() {
            if spaces < *new_top {
                indentation_stack.pop();
                dedent_tokens.push(Token::new_dedent(current_pos));
            } else if spaces == *new_top {
                break;
            } else {
                // Illegal state: Indentation error
                let error = LexError {
                    position: current_pos,
                    reason: format!(
                        "Indentation error: expected {} spaces, found {}",
                        new_top, spaces
                    ),
                };
                return LexResult::Err(error);
            }
        }

        return LexResult::Multiple(dedent_tokens, current_pos + spaces);
    } else {
        // Same indentation level
        return next_token(chars, current_pos + spaces, indentation_stack, false);
    }
}

/// Returns the char at `pos`
fn peek(input: &Chars, pos: usize) -> char {
    let result = input.get(pos).unwrap_or(&'\0');
    *result
}

/// Whether there is still input based on `current_pos`
fn has_input(input: &Chars, current_pos: usize) -> bool {
    current_pos < input.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;

    /// Should return an EOF token if the input has no tokens
    #[test]
    fn should_emit_eof() {
        let input = String::from("");
        let tokens = get_tokens(&input).unwrap();
        // a EOF token
        assert_eq!(1, tokens.len());
        let first = tokens.get(0).unwrap();
        assert_eq!(TokenType::EOF, first.token_type);

        let input = String::from("  ");
        let tokens = get_tokens(&input).unwrap();
        // a EOF token
        assert_eq!(1, tokens.len());
        let first = tokens.get(0).unwrap();
        assert_eq!(TokenType::EOF, first.token_type);

        let input = String::from("    ");
        let tokens = get_tokens(&input).unwrap();
        // a EOF token
        assert_eq!(1, tokens.len());
        let first = tokens.get(0).unwrap();
        assert_eq!(TokenType::EOF, first.token_type);
    }

    #[test]
    fn t() {
        let input = String::from("126 ");
        let chars: Vec<char> = input.chars().into_iter().collect();
        let mut indentation_stack = Vec::<usize>::new();

        assert_eq!(4, chars.len());
        assert!(has_input(&chars, 0));

        match next_token(&chars, 0, &mut indentation_stack, true) {
            LexResult::Some(t, _) => {
                assert_eq!("126", t.value)
            }
            _ => {
                panic!()
            }
        }
    }

    /// Should scan numbers
    #[test]
    fn number_test() {
        let input = String::from("126 278.98 0.282398 1789e+1 239.3298e-103");
        let tokens = get_tokens(&input).unwrap();

        let t1 = tokens.get(0).unwrap();
        assert_eq!(TokenType::Number, t1.token_type);
        assert_eq!("126", t1.value);

        let t2 = tokens.get(1).unwrap();
        assert_eq!(TokenType::Number, t2.token_type);
        assert_eq!("278.98", t2.value);

        let t3 = tokens.get(2).unwrap();
        assert_eq!(TokenType::Number, t3.token_type);
        assert_eq!("0.282398", t3.value);

        assert_eq!("1789e+1", tokens.get(3).unwrap().value);
        assert_eq!("239.3298e-103", tokens.get(4).unwrap().value);
        assert_eq!(TokenType::EOF, tokens.get(5).unwrap().token_type);
    }

    #[test]
    fn grouping_sign_test() {
        let input = String::from("( ) { } [ ]");
        let tokens = get_tokens(&input).unwrap();

        let t = tokens.get(0).unwrap();
        assert_eq!(TokenType::LeftParen, t.token_type);
        assert_eq!("(", t.value);

        let t = tokens.get(1).unwrap();
        assert_eq!(TokenType::RightParen, t.token_type);
        assert_eq!(")", t.value);

        let t = tokens.get(2).unwrap();
        assert_eq!(TokenType::LeftBrace, t.token_type);
        assert_eq!("{", t.value);

        let t = tokens.get(3).unwrap();
        assert_eq!(TokenType::RightBrace, t.token_type);
        assert_eq!("}", t.value);

        let t = tokens.get(4).unwrap();
        assert_eq!(TokenType::LeftBracket, t.token_type);
        assert_eq!("[", t.value);

        let t = tokens.get(5).unwrap();
        assert_eq!(TokenType::RightBracket, t.token_type);
        assert_eq!("]", t.value);
    }

    #[test]
    fn should_scan_datatype() {
        let input = String::from("Num");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Datatype, tokens[0].token_type);
    }

    #[test]
    fn should_scan_new_line() {
        let input = String::from("3\n22");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::NewLine, tokens[1].token_type);
    }

    #[test]
    fn should_scan_multiple_new_lines() {
        let input = String::from("3\n\n\n22");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::NewLine, tokens[1].token_type);
        assert_eq!(TokenType::Number, tokens[2].token_type);
    }

    #[test]
    fn should_scan_multiple_new_lines_with_whitespace_in_between() {
        let input = String::from("3\n \n   \n22");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::NewLine, tokens[1].token_type);
        assert_eq!(TokenType::Number, tokens[2].token_type);
    }

    #[test]
    fn should_emit_indent_token() {
        let input = String::from("3\n \n   22");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Number, tokens[0].token_type);
        assert_eq!(TokenType::NewLine, tokens[1].token_type);
        assert_eq!(TokenType::INDENT, tokens[2].token_type);
        assert_eq!(TokenType::Number, tokens[3].token_type);
    }

    #[test]
    fn should_emit_indent_when_indentation_increases() {
        let input = String::from("3\n \n    22\n        111");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Number, tokens[0].token_type);
        assert_eq!(TokenType::NewLine, tokens[1].token_type);
        assert_eq!(TokenType::INDENT, tokens[2].token_type);
        assert_eq!(TokenType::Number, tokens[3].token_type);
        assert_eq!(TokenType::NewLine, tokens[4].token_type);
        assert_eq!(TokenType::INDENT, tokens[5].token_type);
        assert_eq!(TokenType::Number, tokens[6].token_type);
    }

    #[test]
    fn shouldnt_emit_indent_when_indentation_stays() {
        let input = String::from("3\n \n    22\n    111");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Number, tokens[0].token_type);
        assert_eq!(TokenType::NewLine, tokens[1].token_type);
        assert_eq!(TokenType::INDENT, tokens[2].token_type);
        assert_eq!(TokenType::Number, tokens[3].token_type);
        assert_eq!(TokenType::NewLine, tokens[4].token_type);
        assert_eq!(TokenType::Number, tokens[5].token_type);
    }

    #[test]
    fn should_emit_dedent() {
        let input = String::from("3\n \n    22\n111");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Number, tokens[0].token_type);
        assert_eq!(TokenType::NewLine, tokens[1].token_type);
        assert_eq!(TokenType::INDENT, tokens[2].token_type);
        assert_eq!(TokenType::Number, tokens[3].token_type);
        assert_eq!(TokenType::NewLine, tokens[4].token_type);
        assert_eq!(TokenType::DEDENT, tokens[5].token_type);
        assert_eq!(TokenType::Number, tokens[6].token_type);
    }

    #[test]
    fn should_emit_multiple_dedents() {
        let input = String::from("1\n  2\n    3\n  4\n5");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Number, tokens[0].token_type);
        assert_eq!(TokenType::NewLine, tokens[1].token_type);
        assert_eq!(TokenType::INDENT, tokens[2].token_type);
        assert_eq!(TokenType::Number, tokens[3].token_type);
        assert_eq!(TokenType::NewLine, tokens[4].token_type);
        assert_eq!(TokenType::INDENT, tokens[5].token_type);
        assert_eq!(TokenType::Number, tokens[6].token_type);
        assert_eq!(TokenType::NewLine, tokens[7].token_type);
        assert_eq!(TokenType::DEDENT, tokens[8].token_type);
        assert_eq!(TokenType::Number, tokens[9].token_type);
        assert_eq!(TokenType::NewLine, tokens[10].token_type);
        assert_eq!(TokenType::DEDENT, tokens[11].token_type);
    }

    #[test]
    fn should_emit_multiple_dedents_2() {
        let input = String::from("1\n  2\n    3\n4");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Number, tokens[0].token_type);
        assert_eq!(TokenType::NewLine, tokens[1].token_type);
        assert_eq!(TokenType::INDENT, tokens[2].token_type);
        assert_eq!(TokenType::Number, tokens[3].token_type);
        assert_eq!(TokenType::NewLine, tokens[4].token_type);
        assert_eq!(TokenType::INDENT, tokens[5].token_type);
        assert_eq!(TokenType::Number, tokens[6].token_type);
        assert_eq!(TokenType::NewLine, tokens[7].token_type);
        assert_eq!(TokenType::DEDENT, tokens[8].token_type);
        assert_eq!(TokenType::DEDENT, tokens[9].token_type);
    }
}

#[cfg(test)]
mod indentation_tests {
    use super::*;
    use token::TokenType;

    #[test]
    fn should_emit_dedents_on_eof() {
        let input = String::from("1\n  2");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Number, tokens[0].token_type);
        assert_eq!(TokenType::NewLine, tokens[1].token_type);
        assert_eq!(TokenType::INDENT, tokens[2].token_type);
        assert_eq!(TokenType::Number, tokens[3].token_type);
        assert_eq!(TokenType::DEDENT, tokens[4].token_type);
        assert_eq!(TokenType::EOF, tokens[5].token_type);
    }

    #[test]
    fn should_emit_dedents_on_eof_2() {
        let input = String::from("1\n  2\n    3");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Number, tokens[0].token_type);
        assert_eq!(TokenType::NewLine, tokens[1].token_type);
        assert_eq!(TokenType::INDENT, tokens[2].token_type);
        assert_eq!(TokenType::Number, tokens[3].token_type);
        assert_eq!(TokenType::NewLine, tokens[4].token_type);
        assert_eq!(TokenType::INDENT, tokens[5].token_type);
        assert_eq!(TokenType::Number, tokens[6].token_type);
        assert_eq!(TokenType::DEDENT, tokens[7].token_type);
        assert_eq!(TokenType::DEDENT, tokens[8].token_type);
        assert_eq!(TokenType::EOF, tokens[9].token_type);
    }
}
