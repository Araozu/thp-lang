use crate::error_handling::{MistiError, SyntaxError};

mod binding;
mod expression;
mod function_declaration;
mod utils;

pub mod ast;

use crate::lexic::token::Token;
use ast::{Binding, ModuleAST};

use self::ast::TopLevelConstruct;

#[derive(Debug)]
pub enum SyntaxResult {
    ///
    /// A construct has been found
    Ok(TopLevelConstruct),
    ///
    /// No construct was found
    None,
    ///
    /// A construct was found, but there was an error parsing it
    Err(SyntaxError),
}

/// Constructs the Misti AST from a vector of tokens
pub fn construct_ast<'a>(tokens: &'a Vec<Token>) -> Result<ModuleAST, MistiError> {
    let _token_amount = tokens.len();
    let current_pos = 0;

    match next_construct(tokens, current_pos) {
        SyntaxResult::Ok(module) => Ok(ModuleAST {
            bindings: vec![module],
        }),
        SyntaxResult::None => Err(MistiError::Syntax(SyntaxError {
            reason: String::from("PARSER couldn't parse any construction"),
            // FIXME: This should get the position of the _token_ that current_pos points to
            error_start: current_pos,
            error_end: current_pos,
        })),
        SyntaxResult::Err(err) => Err(MistiError::Syntax(err)),
    }
}

fn next_construct<'a>(tokens: &'a Vec<Token>, current_pos: usize) -> SyntaxResult {
    None.or_else(|| binding::try_parse(tokens, current_pos))
        .unwrap_or_else(|| SyntaxResult::None)
}
