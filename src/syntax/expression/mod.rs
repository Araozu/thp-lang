use super::{ast::Expression, ParsingResult};
use crate::lexic::token::Token;

mod comparison;
mod equality;
mod factor;
pub mod function_call_expr;
mod primary;
mod term;
mod unary;

/// Expression is defined in the grammar.
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    // TODO: This must be newline/indentation aware
    equality::try_parse(tokens, pos)
}

#[cfg(test)]
mod tests {}
