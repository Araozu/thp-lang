use crate::{
    error_handling::{error_messages::SYNTAX_INVALID_ARRAY_ACCESS, ErrorContainer, ErrorLabel},
    lexic::token::{Token, TokenType},
    syntax::{
        ast::{functions::FunctionCall, ArrayAccess, Expression},
        functions::arguments_list,
        parseable::Parseable,
        utils::parse_token_type,
        ParsingError, ParsingResult,
    },
};

/// Parses a function call expression.
///
/// ```ebnf
/// function call expr = primary, "(", (arguments list)?, ")"
///                    | primary, "[", expressions, "]"
///                    | primary;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    let (primary_expr, next_pos) = match super::primary::try_parse(tokens, pos) {
        Ok((expr, next_pos)) => (expr, next_pos),
        _ => return Err(ParsingError::Unmatched),
    };

    // Attempt to parse a function call
    // Parse arguments list
    match arguments_list::try_parse(tokens, next_pos) {
        Ok((arguments, next_pos)) => {
            let fun_call = FunctionCall {
                function: Box::new(primary_expr),
                arguments: Box::new(arguments),
            };

            return Ok((Expression::FunctionCall(fun_call), next_pos));
        }
        Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
        _ => {}
    };

    // Attempt to parse an array access
    match try_parse_array_access(tokens, next_pos) {
        Ok(((array_idx, end_pos), next_pos)) => {
            let fun_call = ArrayAccess {
                left_expr: Box::new(primary_expr),
                idx_expr: Box::new(array_idx),
                end_pos,
            };

            return Ok((Expression::ArrayAcccess(fun_call), next_pos));
        }
        Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
        _ => {}
    };

    // Return the parsed primary
    return Ok((primary_expr, next_pos));
}

pub fn try_parse_array_access(
    tokens: &Vec<Token>,
    pos: usize,
) -> ParsingResult<(Expression, usize)> {
    let (_token, next_pos) = parse_token_type(&tokens, pos, TokenType::LeftBracket)?;

    // parse an expression
    let (exp, next_pos) = match Expression::try_parse(tokens, next_pos) {
        Ok(t) => t,
        Err(ParsingError::Mismatch(e)) => {
            let label = ErrorLabel {
                message: String::from("Expected an expression for this array access"),
                start: e.position,
                end: e.get_end_position(),
            };
            let econtainer = ErrorContainer {
                error_code: SYNTAX_INVALID_ARRAY_ACCESS,
                error_offset: e.position,
                labels: vec![label],
                note: None,
                help: None,
            };
            return Err(ParsingError::Err(econtainer));
        }
        Err(ParsingError::Unmatched) => {
            //
            let label = ErrorLabel {
                message: String::from("Expected an expression for this array access"),
                start: pos,
                end: pos + 1,
            };
            let econtainer = ErrorContainer {
                error_code: SYNTAX_INVALID_ARRAY_ACCESS,
                error_offset: pos,
                labels: vec![label],
                note: None,
                help: None,
            };
            return Err(ParsingError::Err(econtainer));
        }
        Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
    };

    // parse the closing bracket
    let (closed_bracket, next) = match parse_token_type(tokens, next_pos, TokenType::RightBracket) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    return Ok(((exp, closed_bracket.get_end_position()), next));
}

#[cfg(test)]
mod test {}
