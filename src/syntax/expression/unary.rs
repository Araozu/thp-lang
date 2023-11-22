use crate::{
    lexic::token::Token,
    syntax::{ast::Expression, expression::primary, ParseResult},
};

/// Parses an unary expression.
///
/// ```ebnf
/// unary = ("!" | "-"), expression
///       | primary;
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
        _ => primary::try_parse(tokens, pos),
    }
}

