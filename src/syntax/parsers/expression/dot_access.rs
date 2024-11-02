use crate::{
    lexic::token::Token,
    syntax::{
        ast::Expression, parsers::expression::utils::try_binary_op, ParsingError, ParsingResult,
    },
};

/// Parses a dot access
///
/// ```ebnf
/// dot_access = unary, (("."), unary)*;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    let (unary, next_pos) = match super::unary::try_parse(tokens, pos) {
        Ok((expr, next_pos)) => (expr, next_pos),
        _ => return Err(ParsingError::Unmatched),
    };

    parse_many(tokens, next_pos, unary, 0)
}

fn parse_many<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
    prev_expr: Expression<'a>,
    indentation_level: u32,
) -> ParsingResult<'a, Expression<'a>> {
    // (("/" | "*" | "%"), unary)*
    try_binary_op(
        tokens,
        pos,
        prev_expr,
        vec!["."],
        indentation_level,
        |tokens, next_pos, prev_expr, token, indent_count: u32| {
            // match next
            match super::unary::try_parse(tokens, next_pos) {
                Ok((expr, next_pos)) => {
                    let expr =
                        Expression::BinaryOperator(Box::new(prev_expr), Box::new(expr), &token);

                    parse_many(tokens, next_pos, expr, indentation_level + indent_count)
                }
                _ => return Err(ParsingError::Unmatched),
            }
        },
    )
}
