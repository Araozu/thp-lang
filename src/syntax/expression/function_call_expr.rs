use crate::{
    lexic::token::Token,
    syntax::{
        ast::{
            functions::{ArgumentsList, FunctionCall},
            Expression,
        },
        functions::arguments_list,
        ParseResult,
    },
};

/// Parses a function call expression.
///
/// ```ebnf
/// function call expr = primary, "(", (arguments list)?, ")"
///                    | primary;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParseResult<Expression, ()> {
    let (primary_expr, next_pos) = match super::primary::try_parse(tokens, pos) {
        ParseResult::Ok(expr, next_pos) => (expr, next_pos),
        _ => return ParseResult::Unmatched,
    };

    // Parse arguments list
    let (arguments, next_pos) = match arguments_list::try_parse(tokens, next_pos) {
        ParseResult::Ok(args, next) => (args, next),
        ParseResult::Err(err) => return ParseResult::Err(err),
        _ => {
            return ParseResult::Ok(primary_expr, next_pos);
        }
    };

    let fun_call = FunctionCall {
        function: Box::new(primary_expr),
        arguments: Box::new(arguments),
    };

    ParseResult::Ok(Expression::FunctionCall(fun_call), next_pos)
}

#[cfg(test)]
mod test {}
