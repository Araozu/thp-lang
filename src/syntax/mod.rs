use super::token::Token;

mod expression;
mod types;

/// Constructs the Misti AST from a vector of tokens
pub fn construct_ast(_tokens: Vec<Token>) -> Result<types::ModuleAST, String> {
    Err(String::from("NOT IMPLEMENTED"))
}


