use crate::lexic::token::{Token, TokenType};

use super::{ParsingError, ParsingResult};

pub trait Tokenizer {
    fn get_significant<'a>(&'a self, index: usize) -> Option<(&'a Token, usize)>;
    fn code_position_from_idx(&self, idx: usize) -> usize;
}

impl Tokenizer for Vec<Token> {
    /// Returns the first non whitespace token at index & the position the found token
    fn get_significant<'a>(&'a self, index: usize) -> Option<(&'a Token, usize)> {
        let mut current_pos = index;

        // Ignore all whitespace, newlines and comments
        loop {
            match self.get(current_pos) {
                Some(token) => {
                    if token.token_type == TokenType::INDENT
                        || token.token_type == TokenType::DEDENT
                        || token.token_type == TokenType::NewLine
                        || token.token_type == TokenType::Comment
                        || token.token_type == TokenType::MultilineComment
                    {
                        current_pos += 1;
                    } else {
                        return Some((token, current_pos));
                    }
                }
                None => return None,
            }
        }
    }

    /// Returns the position in the code from the token idx.
    ///
    /// If the token at `idx` exists, returns `tokens[idx].position`.
    ///
    /// Otherwise returns `tokens[idx - 1].get_end_position()`
    fn code_position_from_idx(&self, idx: usize) -> usize {
        // try to get the token at idx
        match self.get(idx) {
            Some(t) if t.token_type == TokenType::EOF => {
                // If idx points at EOF, return the end position of the previous token
                // This shouldnt fail
                self[idx - 1].get_end_position()
            }
            Some(t) => t.position,
            None => {
                // this should never happen.
                // the token stream always ends with an EOF token,
                // and the parser should never be able to go
                // to a position after that EOF token
                unreachable!("Compiler error: Tried to get an out of bound token. This means that somewhere a token beyond EOF was requested.")
            }
        }
    }
}

/// Expects the token at `pos` to be an operator of value `operator`. Doesn't ignore whitespace or newlines
pub fn try_operator(tokens: &Vec<Token>, pos: usize, operator: String) -> ParsingResult<&Token> {
    match tokens.get(pos) {
        Some(t) if t.token_type == TokenType::Operator && t.value == operator => Ok((t, pos + 1)),
        Some(t) if t.token_type == TokenType::NewLine || t.token_type == TokenType::EOF => {
            Err(ParsingError::Unmatched)
        }
        Some(t) => Err(ParsingError::Mismatch(t)),
        None => Err(ParsingError::Unmatched),
    }
}

/// Expects the token at `pos` to be of type `token_type`, and returns the token and the next position.
///
/// Ignores all whitespace, newlines and comments.
///
/// Only returns: Ok, Unmatched, Mismatched
pub fn parse_token_type(
    tokens: &Vec<Token>,
    pos: usize,
    token_type: TokenType,
) -> ParsingResult<&Token> {
    let mut current_pos = pos;

    // Ignore all whitespace, newlines and semicolons
    while let Some(t) = tokens.get(current_pos) {
        if t.token_type == TokenType::INDENT
            || t.token_type == TokenType::DEDENT
            || t.token_type == TokenType::NewLine
            || t.token_type == TokenType::Comment
            || t.token_type == TokenType::MultilineComment
        {
            current_pos += 1;
        } else {
            break;
        }
    }

    match tokens.get(current_pos) {
        Some(t) if t.token_type == token_type => Ok((t, current_pos + 1)),
        // TODO: Why are we checking if the token is NewLine here? Arent all newlines filtered
        // above?
        Some(t) if t.token_type == TokenType::EOF || t.token_type == TokenType::NewLine => {
            Err(ParsingError::Unmatched)
        }
        Some(t) => Err(ParsingError::Mismatch(t)),
        None => Err(ParsingError::Unmatched),
    }
}

/// Expects the token at `pos` to be a terminator (newline or eof)
///
/// Ignores indentation, newlines and comments.
///
/// Only returns: Ok or Unmatched.
pub fn parse_terminator(tokens: &Vec<Token>, pos: usize) -> ParsingResult<()> {
    let mut current_pos = pos;

    // Ignore all whitespace, newlines and semicolons
    while let Some(t) = tokens.get(current_pos) {
        if t.token_type == TokenType::INDENT
            || t.token_type == TokenType::DEDENT
            || t.token_type == TokenType::Comment
            || t.token_type == TokenType::MultilineComment
        {
            current_pos += 1;
        } else {
            break;
        }
    }

    match tokens.get(current_pos) {
        Some(t) if t.token_type == TokenType::EOF || t.token_type == TokenType::NewLine => {
            Ok(((), current_pos + 1))
        }
        Some(t) => Err(ParsingError::Mismatch(t)),
        None => unreachable!("Stream of tokens finished before getting an EOF"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexic::{get_tokens, token::TokenType},
        syntax::{
            parseable::ParsingError,
            utils::{parse_token_type, Tokenizer},
        },
    };

    use super::try_operator;

    #[test]
    fn test_1() {
        let input = String::from("");
        let tokens = get_tokens(&input).unwrap();
        assert_eq!(1, tokens.len());

        match try_operator(&tokens, 10, "+".into()) {
            Ok(_) => panic!("Expected an error"),
            Err(error) => match error {
                ParsingError::Unmatched => {
                    assert!(true);
                }
                _ => panic!(
                    "Expected an error due to incorrect position, got {:?}",
                    error
                ),
            },
        }
    }

    #[test]
    fn test_2() {
        let input = String::from("");
        let tokens = get_tokens(&input).unwrap();
        assert_eq!(1, tokens.len());

        match parse_token_type(&tokens, 10, TokenType::Operator) {
            Ok(_) => panic!("Expected an error"),
            Err(error) => match error {
                ParsingError::Unmatched => {
                    assert!(true);
                }
                _ => panic!(
                    "Expected an error due to incorrect position, got {:?}",
                    error
                ),
            },
        }
    }

    #[test]
    fn test_3() {
        let input = String::from("");
        let tokens = get_tokens(&input).unwrap();
        assert_eq!(1, tokens.len());

        match tokens.get_significant(10) {
            Some(_) => panic!("Expected a None"),
            None => {}
        }
    }
}
