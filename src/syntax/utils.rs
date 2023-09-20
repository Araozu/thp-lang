use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
    utils::Result3,
};

use super::SyntaxResult;

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

pub fn try_operator_w<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
    operator: String,
    error_message: String,
    prev_token: &Token,
) -> Result<(&'a Token, usize), Option<SyntaxResult>> {
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
        Some(t) if t.token_type == TokenType::Operator && t.value == operator => {
            Ok((t, current_pos + 1))
        }
        Some(t) if t.token_type == TokenType::NewLine || t.token_type == TokenType::EOF => {
            Err(Some(SyntaxResult::Err(SyntaxError {
                reason: error_message,
                error_start: prev_token.position,
                error_end: prev_token.get_end_position(),
            })))
        }
        Some(t) => Err(Some(SyntaxResult::Err(SyntaxError {
            reason: error_message,
            error_start: t.position,
            error_end: t.get_end_position(),
        }))),
        None => Err(Some(SyntaxResult::Err(SyntaxError {
            reason: error_message,
            error_start: prev_token.position,
            error_end: prev_token.get_end_position(),
        }))),
    }
}

/// Ignores all whitespace and newlines
pub fn expect_token_w<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
    token_type: TokenType,
    error_message: String,
    prev_token: &Token,
) -> Result<(&'a Token, usize), Option<SyntaxResult>> {
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
        Some(t) if t.token_type == token_type => Ok((t, current_pos + 1)),
        Some(t) if t.token_type == TokenType::EOF || t.token_type == TokenType::NewLine => {
            Err(Some(SyntaxResult::Err(SyntaxError {
                reason: error_message,
                error_start: prev_token.position,
                error_end: prev_token.get_end_position(),
            })))
        }
        Some(t) => Err(Some(SyntaxResult::Err(SyntaxError {
            reason: error_message,
            error_start: t.position,
            error_end: t.get_end_position(),
        }))),
        None => Err(Some(SyntaxResult::Err(SyntaxError {
            reason: error_message,
            error_start: prev_token.position,
            error_end: prev_token.get_end_position(),
        }))),
    }
}
