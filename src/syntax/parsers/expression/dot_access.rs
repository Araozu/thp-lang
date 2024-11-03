use crate::{
    lexic::token::Token,
    syntax::{ast::Expression, ParsingError, ParsingResult},
};

use super::utils::parse_many;

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

    parse_many(tokens, next_pos, unary, 0, &vec![".", "?.", "!."])
}
