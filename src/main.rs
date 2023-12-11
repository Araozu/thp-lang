
// Module to handle the repl and its compilation
mod repl;
// Module to handle file compilation
mod file;
// Module to handle lexical analysis
mod syntax;
// Module to handle syntactic analysis
mod lexic;
// Transforms an AST to JS
mod codegen;
mod utils;

mod error_handling;

fn get_copyright() -> String {
    let crate_version = env!("CARGO_PKG_VERSION");
    format!("The THP compiler, linter & formatter, v{}", crate_version,)
}

fn main() {
    println!("{}", get_copyright());
    println!("Rewriting CLI...");
}
