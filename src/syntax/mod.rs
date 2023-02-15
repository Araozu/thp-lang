use super::token::Token;

mod expression;
mod binding;
use super::ast_types;

use ast_types::ModuleAST;

/// Constructs the Misti AST from a vector of tokens
pub fn construct_ast<'a>(tokens: &'a Vec<Token>) -> Result<ModuleAST<'a>, String> {
    let maybe_binding = binding::try_parse(tokens, 0);

    match maybe_binding {
        Some(binding) => {
            Ok(ModuleAST {
                bindings: vec![binding]
            })
        }
        None => {
            Err(String::from("Syntax error."))
        }
    }
}


