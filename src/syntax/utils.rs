use crate::{
    lexic::token::{Token, TokenType},
    utils::Result3,
};

use super::ParseResult;

/// Expects the token at `pos` to be of type `token_type`
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

/// Expects the token at `pos` to be of type `token_type`
pub fn parse_token_type(
    tokens: &Vec<Token>,
    pos: usize,
    token_type: TokenType,
) -> ParseResult<&Token, &Token> {
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
        Some(t) => ParseResult::Mismatch(t),
        None => ParseResult::Unmatched,
    }
}
