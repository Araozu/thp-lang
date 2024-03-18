use crate::lexic::token::{Token, TokenType};

use super::{ParsingError, ParsingResult};

pub trait Tokenizer {
    fn get_significant<'a>(&'a self, index: usize) -> Option<(&'a Token, usize)>;
}

impl Tokenizer for Vec<Token> {
    /// Returns the first non whitespace token at index & the position the found token
    fn get_significant<'a>(&'a self, index: usize) -> Option<(&'a Token, usize)> {
        let mut current_pos = index;

        // Ignore all whitespace and newlines
        loop {
            match self.get(current_pos) {
                Some(token) => {
                    if token.token_type == TokenType::INDENT
                        || token.token_type == TokenType::DEDENT
                        || token.token_type == TokenType::NewLine
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
/// Ignores all whitespace and newlines.
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
        {
            current_pos += 1;
        } else {
            break;
        }
    }

    match tokens.get(current_pos) {
        Some(t) if t.token_type == token_type => Ok((t, current_pos + 1)),
        Some(t) if t.token_type == TokenType::EOF || t.token_type == TokenType::NewLine => {
            Err(ParsingError::Unmatched)
        }
        Some(t) => Err(ParsingError::Mismatch(t)),
        None => Err(ParsingError::Unmatched),
    }
}


