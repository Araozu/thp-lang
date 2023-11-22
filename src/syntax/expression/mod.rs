use super::{ast::Expression, ParseResult};
use crate::lexic::token::Token;

mod comparison;
mod equality;
mod factor;
mod primary;
mod term;
mod unary;

/// Expression is defined in the grammar.
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParseResult<Expression, ()> {
    return equality::try_parse(tokens, pos);
}
