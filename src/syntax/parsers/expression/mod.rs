use super::super::{ast::Expression, ParsingResult};
use crate::{lexic::token::Token, syntax::parseable::Parseable};

mod comparison;
mod equality;
mod factor;
pub mod function_call_expr;
mod primary;
mod term;
mod unary;
mod utils;
mod array;

impl<'a> Parseable<'a> for Expression<'a> {
    type Item = Expression<'a>;

    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        equality::try_parse(tokens, current_pos)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexic::get_tokens;

    use super::*;

    #[test]
    fn should_parse_expression_w_indentation_1() {
        let tokens = get_tokens(&String::from("a\n  == b")).unwrap();
        let (expr, _) = Expression::try_parse(&tokens, 0).unwrap();
        match expr {
            Expression::BinaryOperator(_e1, _e2, _op) => {}
            _ => panic!("Expected a binary operation"),
        }
    }
}
