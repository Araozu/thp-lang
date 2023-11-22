use crate::{
    lexic::token::Token,
    syntax::{ast::Expression, ParseResult},
};

/// Parses a factor expression.
///
/// ```ebnf
/// factor = unary, (("/" | "*"), unary)*;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParseResult<Expression, ()> {
    let (unary, next_pos) = match super::unary::try_parse(tokens, pos) {
        ParseResult::Ok(expr, next_pos) => (expr, next_pos),
        _ => return ParseResult::Unmatched,
    };

    parse_many(tokens, next_pos, unary)
}

fn parse_many(
    tokens: &Vec<Token>,
    pos: usize,
    prev_expr: Expression,
) -> ParseResult<Expression, ()> {
    // (("/" | "*"), unary)*

    match tokens.get(pos) {
        Some(token) if token.value == "/" || token.value == "*" => {
            match super::unary::try_parse(tokens, pos + 1) {
                ParseResult::Ok(expr, next_pos) => {
                    let expr = Expression::BinaryOperator(
                        Box::new(prev_expr),
                        Box::new(expr),
                        Box::new(token.value.clone()),
                    );

                    parse_many(tokens, next_pos, expr)
                }
                _ => ParseResult::Unmatched,
            }
        }
        _ => ParseResult::Ok(prev_expr, pos),
    }
}
