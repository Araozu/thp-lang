use crate::lexic::token::Token;
use crate::lexic::token::TokenType::{NewLine, DEDENT, INDENT};
use crate::syntax::ast::Expression;
use crate::syntax::parseable::ParsingResult;

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
    operators: Vec<&str>,
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
    let pos = match (tokens.get(pos), tokens.get(pos + 1)) {
        // New indentation level
        (Some(t1), Some(t2)) if t1.token_type == NewLine && t2.token_type == INDENT => {
            indent_count += 1;
            pos + 2
        }
        // when indented, ignore newlines
        (Some(t), _) if t.token_type == NewLine && indentation_level > 0 => pos + 1,
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
    let pos = match (tokens.get(pos), tokens.get(pos + 1)) {
        // New indentation level
        (Some(t1), Some(t2)) if t1.token_type == NewLine && t2.token_type == INDENT => {
            indent_count += 1;
            pos + 2
        }
        // when indented, ignore newlines
        (Some(t), _) if t.token_type == NewLine && indentation_level > 0 => pos + 1,
        // let other handlers handle this
        _ => pos,
    };

    // run the rest of the logic
    let (new_expr, mut next_pos) = match fun(tokens, pos, prev_expr, matched_token, indent_count) {
        Ok((e, n)) => (e, n),
        x => return x,
    };

    // handle the possible dedentation before/after the operator
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
