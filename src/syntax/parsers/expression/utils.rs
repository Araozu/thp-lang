use crate::lexic::token::Token;
use crate::lexic::token::TokenType::{NewLine, DEDENT, INDENT};
use crate::syntax::ast::Expression;
use crate::syntax::parseable::{ParsingError, ParsingResult};

/// Parses a binary operator, handles indentation and runs a function on it.
///
/// First, handles indentation before the binary operator. Then, tries to
/// parse the binary operator. Then, handles indentation after the binary
/// operator.
///
/// After this runs the function `fun`. Finishes by handling dedentation
/// parsed in the previous phase.
pub fn try_binary_op<'a, F>(
    tokens: &'a Vec<Token>,
    original_pos: usize,
    prev_expr: Expression<'a>,
    operators: &Vec<&str>,
    indentation_level: u32,
    fun: F,
) -> ParsingResult<'a, Expression<'a>>
where
    F: FnOnce(
        &'a Vec<Token>,
        usize,
        Expression<'a>,
        &'a Token,
        u32,
    ) -> ParsingResult<'a, Expression<'a>>,
{
    let mut indent_count = 0;
    let pos = original_pos;

    // handle possible opening indentation
    let pos = match tokens.get(pos) {
        // New indentation level
        Some(t2) if t2.token_type == INDENT => {
            indent_count += 1;
            pos + 1
        }
        // when indented, ignore newlines
        Some(t) if t.token_type == NewLine && indentation_level > 0 => pos + 1,
        // let other handlers handle this
        _ => pos,
    };

    // try to parse any of the binary operators
    let (matched_token, pos) = match tokens.get(pos) {
        Some(token) if operators.contains(&token.value.as_str()) => (token, pos + 1),
        // If not matched, return the existing expression
        _ => return Ok((prev_expr, original_pos)),
    };

    // handle possible closing indentation
    let pos = match tokens.get(pos) {
        // New indentation level
        Some(t2) if t2.token_type == INDENT => {
            indent_count += 1;
            pos + 1
        }
        // when indented, ignore newlines
        Some(t) if t.token_type == NewLine && indentation_level > 0 => pos + 1,
        // let other handlers handle this
        _ => pos,
    };

    // run the rest of the logic
    let (new_expr, mut next_pos) = match fun(tokens, pos, prev_expr, matched_token, indent_count) {
        Ok((e, n)) => (e, n),
        x => return x,
    };

    // handle dedentation before/after the operator
    for _ in 0..indent_count {
        // expect a DEDENT for each INDENT matched
        match tokens.get(next_pos) {
            // continue
            Some(t) if t.token_type == DEDENT => {}
            _ => unreachable!(
                "Illegal parser state: Expected DEDENT (count: {})",
                indent_count
            ),
        };

        next_pos += 1;
    }

    Ok((new_expr, next_pos))
}

pub fn parse_many<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
    prev_expr: Expression<'a>,
    indentation_level: u32,
    operators: &Vec<&str>,
) -> ParsingResult<'a, Expression<'a>> {
    // comparison = term, ((">" | ">=" | "<" | "<="), term)*;
    try_binary_op(
        tokens,
        pos,
        prev_expr,
        operators,
        indentation_level,
        |tokens, next_pos, prev_expr, token, indent_count: u32| match super::term::try_parse(
            tokens, next_pos,
        ) {
            Ok((expr, next_pos)) => {
                let expr = Expression::BinaryOperator(Box::new(prev_expr), Box::new(expr), &token);

                parse_many(
                    tokens,
                    next_pos,
                    expr,
                    indentation_level + indent_count,
                    operators,
                )
            }
            _ => return Err(ParsingError::Unmatched),
        },
    )
}
