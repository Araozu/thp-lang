use super::token::Token;

mod expression;
mod val_binding;
mod types;

use types::ModuleAST;

/// Constructs the Misti AST from a vector of tokens
pub fn construct_ast<'a>(_tokens: Vec<Token>) -> Result<ModuleAST<'a>, String> {
    Err(String::from("NOT IMPLEMENTED"))
}


