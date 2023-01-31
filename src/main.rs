use chrono::{prelude::Utc, Datelike};
use clap::{Parser, Subcommand};

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


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// compiles a single file
    C {
        /// File to compile
        file: String,

        /// File to write the JS code to
        output: String,
    },
    /// Starts the REPL
    R {}
}


const VERSION: &str = "0.0.1";

fn get_copyright() -> String {
    let year = Utc::now().year();

    format!("Misti {}\nCopyright (c) {} Fernando Enrique Araoz Morales\n", VERSION, year)
}

/// # Misti
/// 
/// Usage:
/// - `misti` : Starts the compiler in watch mode
/// - `misti w, --watch, -w` : Starts the compiler in watch mode
/// - `misti -c FILE -o OUTPUT` : Compiles FILE and writes the result in OUTPUT
fn main() {
    println!("{}", get_copyright());

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::C { file: _, output: _ }) => {
            println!("Compile a file: Not implemented")
        }
        Some(Commands::R {  }) => {
            let _ = repl::run();
        }
        None => {
            println!("Compile in watch mode: Not implemented")
        }
    }

}

