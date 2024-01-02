use crate::{
    lexic::token::Token,
    syntax::{ast::Expression, ParseResult},
};

use super::function_call_expr;

/// Parses an unary expression.
///
/// ```ebnf
/// unary = ("!" | "-"), expression
///       | function call expr;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParseResult<Expression, ()> {
    match tokens.get(pos) {
        Some(token) if token.value == "!" || token.value == "-" => {
            match super::try_parse(tokens, pos + 1) {
                ParseResult::Ok(expression, next_pos) => ParseResult::Ok(
                    Expression::UnaryOperator(Box::new(token.value.clone()), Box::new(expression)),
                    next_pos,
                ),
                _ => ParseResult::Unmatched,
            }
        }
        _ => function_call_expr::try_parse(tokens, pos),
    }
}
