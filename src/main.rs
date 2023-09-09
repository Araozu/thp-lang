use clap::{Parser, Subcommand};

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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compiles a single file
    C {
        /// File to compile
        file: String,
    },
    /// Starts the REPL
    R {},
}

const VERSION: &str = "0.0.5";

fn get_copyright() -> String {
    format!(
        "THP {}\nCopyright (c) 2023 Fernando Enrique Araoz Morales\n",
        VERSION,
    )
}

/// # Misti
///
/// Usage:
/// - `misti` : Starts the compiler in watch mode
/// - `misti w, --watch, -w` : Starts the compiler in watch mode
/// - `misti c FILE` : Compiles FILE and writes the result in the same directory
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::C {
            file: input,
        }) => file::compile_file(input),
        Some(Commands::R {}) => {
            println!("{}", get_copyright());
            let _ = repl::run();
        }
        None => {
            println!("Compile in watch mode: Not implemented")
        }
    }
}
