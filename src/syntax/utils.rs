use crate::{
    lexic::token::{Token, TokenType},
    utils::Result3,
};

use super::ParseResult;

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

/// Expects the token at `pos` to be of type `token_type`. Doesn't ignore whitespace or newlines
pub fn try_token_type(tokens: &Vec<Token>, pos: usize, token_type: TokenType) -> Result3<&Token> {
    match tokens.get(pos) {
        Some(t) if t.token_type == token_type => Result3::Ok(t),
        Some(t) if t.token_type == TokenType::EOF || t.token_type == TokenType::NewLine => {
            Result3::None
        }
        Some(t) => Result3::Err(t),
        None => Result3::None,
    }
}

/// Expects the token at `pos` to be an operator of value `operator`. Doesn't ignore whitespace or newlines
pub fn try_operator(tokens: &Vec<Token>, pos: usize, operator: String) -> Result3<&Token> {
    match tokens.get(pos) {
        Some(t) if t.token_type == TokenType::Operator && t.value == operator => Result3::Ok(t),
        Some(t) if t.token_type == TokenType::NewLine || t.token_type == TokenType::EOF => {
            Result3::None
        }
        Some(t) => Result3::Err(t),
        None => Result3::None,
    }
}

/// Expects the token at `pos` to be of type `token_type`, ignoring all whitespace & newlines
pub fn parse_token_type(
    tokens: &Vec<Token>,
    pos: usize,
    token_type: TokenType,
) -> ParseResult<&Token> {
    let mut current_pos = pos;

    // Ignore all whitespace and newlines
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
        Some(t) if t.token_type == token_type => ParseResult::Ok(t, current_pos + 1),
        Some(t) if t.token_type == TokenType::EOF || t.token_type == TokenType::NewLine => {
            ParseResult::Unmatched
        }
        Some(t) => ParseResult::Mismatch(t.clone()),
        None => ParseResult::Unmatched,
    }
}
