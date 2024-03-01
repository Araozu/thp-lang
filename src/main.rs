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
// Transforms an AST to JS
mod codegen;
mod utils;

mod error_handling;

fn main() -> Result<(), ()> {
    cli::run_cli()
}
