use crate::ast_types::Binding;
use crate::error_handling::SyntaxError;

use super::token::Token;

mod binding;
mod expression;
use super::ast_types;

use ast_types::ModuleAST;

pub enum SyntaxResult<'a> {
    ///
    /// A construct has been found
    Ok(Binding<'a>),
    ///
    /// No construct was found
    None,
    ///
    /// A construct was found, but there was an error parsing it
    Err(SyntaxError),
}

/// Constructs the Misti AST from a vector of tokens
pub fn construct_ast<'a>(tokens: &'a Vec<Token>) -> Result<ModuleAST<'a>, SyntaxError> {
    let _token_amount = tokens.len();
    let mut current_pos = 0;

    match next_construct(tokens, current_pos) {
        SyntaxResult::Ok(module) => Ok(ModuleAST {
            bindings: vec![module],
        }),
        SyntaxResult::None => Err(SyntaxError {
            reason: String::from("PARSER couldn't parse any construction"),
            // FIXME: This should get the position of the _token_ that current_pos points to
            error_start: current_pos,
            error_end: current_pos,
        }),
        SyntaxResult::Err(err) => Err(err),
    }
}

fn next_construct<'a>(tokens: &'a Vec<Token>, current_pos: usize) -> SyntaxResult {
    None.or_else(|| binding::try_parse(tokens, 0))
        .unwrap_or_else(|| {
            SyntaxResult::Err(SyntaxError {
                reason: String::from("Unrecognized token"),
                // FIXME: This should get the position of the _token_ that current_pos points to
                error_start: current_pos,
                error_end: current_pos,
            })
        })
}
