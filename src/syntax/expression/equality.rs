use crate::{
    lexic::token::Token,
    syntax::{ast::Expression, ParsingError, ParsingResult},
};

/// Parses a factor expression.
///
/// ```ebnf
/// equality = comparison, (("==" | "!="), comparison )*;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    let (comparison, next_pos) = match super::comparison::try_parse(tokens, pos) {
        Ok((expr, next_pos)) => (expr, next_pos),
        _ => return Err(ParsingError::Unmatched),
    };

    parse_many(tokens, next_pos, comparison)
}

fn parse_many<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
    prev_expr: Expression<'a>,
) -> ParsingResult<'a, Expression<'a>> {
    // equality = comparison, (("==" | "!="), comparison )*;

    match tokens.get(pos) {
        Some(token) if token.value == "==" || token.value == "!=" => {
            match super::comparison::try_parse(tokens, pos + 1) {
                Ok((expr, next_pos)) => {
                    let expr = Expression::BinaryOperator(
                        Box::new(prev_expr),
                        Box::new(expr),
                        &token.value,
                    );

                    parse_many(tokens, next_pos, expr)
                }
                _ => Err(ParsingError::Unmatched),
            }
        }
        _ => Ok((prev_expr, pos)),
    }
}
