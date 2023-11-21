use super::{ast::Expression, functions::function_call, ParseResult};
use crate::lexic::token::{Token, TokenType};

mod equality;
mod primary;

/// Expression is defined in the grammar.
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParseResult<Expression, ()> {
    match function_call::try_parse(tokens, pos) {
        super::ParseResult::Ok(function_call, next_pos) => {
            return ParseResult::Ok::<_, ()>(Expression::FunctionCall(function_call), next_pos)
        }
        _ => {}
    };

    match tokens.get(pos) {
        Some(token) => match token.token_type {
            TokenType::Number => {
                ParseResult::Ok(Expression::Number(Box::new(token.value.clone())), pos + 1)
            }
            TokenType::String => {
                ParseResult::Ok(Expression::String(Box::new(token.value.clone())), pos + 1)
            }
            TokenType::Identifier if token.value == "true" || token.value == "false" => {
                ParseResult::Ok(Expression::Boolean(token.value == "true"), pos + 1)
            }
            TokenType::Identifier => ParseResult::Ok(
                Expression::Identifier(Box::new(token.value.clone())),
                pos + 1,
            ),
            _ => ParseResult::Unmatched,
        },
        None => ParseResult::Unmatched,
    }
}

