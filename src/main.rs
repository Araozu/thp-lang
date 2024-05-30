// Module to handle the CLI
mod cli;

// Module to handle the repl and its compilation
mod repl;
// Module to handle file compilation
mod file;
// Module to handle lexical analysis
mod syntax;
// Module to handle syntactic analysis
mod lexic;
// Module to handle semantic analysis
mod semantic;
// Defines the PHP AST
mod php_ast;
// Transforms an AST to JS
mod codegen;

mod error_handling;

fn main() {
    match cli::run_cli() {
        Ok(_) => (),
        Err(_) => std::process::exit(1),
    }
}
