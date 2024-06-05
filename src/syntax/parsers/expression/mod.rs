use super::super::{ast::Expression, ParsingResult};
use crate::{lexic::token::Token, syntax::parseable::Parseable};

mod comparison;
mod equality;
mod factor;
pub mod function_call_expr;
mod primary;
mod term;
mod unary;

impl<'a> Parseable<'a> for Expression<'a> {
    type Item = Expression<'a>;

    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        // TODO: This must be newline/indentation aware
        equality::try_parse(tokens, current_pos)
    }
}

#[cfg(test)]
mod tests {}
