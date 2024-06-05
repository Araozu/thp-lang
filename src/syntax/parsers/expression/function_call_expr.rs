use crate::{
    lexic::token::Token,
    syntax::{
        ast::{functions::FunctionCall, Expression},
        functions::arguments_list,
        ParsingError, ParsingResult,
    },
};

/// Parses a function call expression.
///
/// ```ebnf
/// function call expr = primary, "(", (arguments list)?, ")"
///                    | primary;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    let (primary_expr, next_pos) = match super::primary::try_parse(tokens, pos) {
        Ok((expr, next_pos)) => (expr, next_pos),
        _ => return Err(ParsingError::Unmatched),
    };

    // Parse arguments list
    let (arguments, next_pos) = match arguments_list::try_parse(tokens, next_pos) {
        Ok((args, next)) => (args, next),
        Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
        _ => {
            return Ok((primary_expr, next_pos));
        }
    };

    let fun_call = FunctionCall {
        function: Box::new(primary_expr),
        arguments: Box::new(arguments),
    };

    Ok((Expression::FunctionCall(fun_call), next_pos))
}

#[cfg(test)]
mod test {}
