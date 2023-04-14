// Module to handle the repl and its compilation
mod repl;
// Defines the types of tokens and provides functions to create them
mod token;
// Module to handle lexical analysis
mod syntax;
// Module to handle syntactic analysis
mod lexic;
// Module to handle semantic analysis
mod semantic;
// Defines the AST
mod ast_types;
// Defines the Symbol table and operations within
mod symbol_table;
// Transforms an AST to JS
mod codegen;
mod utils;

mod error_handling;

use error_handling::MistiError;
use token::Token;

pub use token::TokenType;

pub fn tokenize(input: &String) -> Result<Vec<Token>, MistiError> {
    lexic::get_tokens(input)
}

pub fn repl() {
    let _ = repl::run();
}
