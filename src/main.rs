use std::io;
use chrono::{prelude::Utc, Datelike};

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

mod error_handling;

const VERSION: &str = "0.0.1";

fn get_copyright() -> String {
    let year = Utc::now().year();

    format!("Misti {}\nCopyright (c) {} Fernando Enrique Araoz Morales\n", VERSION, year)
}

/// # Misti
/// 
/// Usage:
/// - `misti` : Compiles the current project according to the settings in the misti.json file
/// - `misti --watch, -w` : Starts the compiler in watch mode
/// - `misti -i FILE -o OUTPUT` : Compiles FILE and writes the result in OUTPUT
fn main() -> io::Result<()> {
    print!("{}", get_copyright());
    repl::run()
}

