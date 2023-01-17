use super::token::Token;

mod expression;
mod val_binding;
mod types;

use types::ModuleAST;

/// Constructs the Misti AST from a vector of tokens
pub fn construct_ast<'a>(tokens: &'a Vec<Token>) -> Result<ModuleAST<'a>, String> {
    let maybe_binding = val_binding::try_parse(tokens, 0);

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


