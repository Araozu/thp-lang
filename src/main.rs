use std::io;
use chrono::{prelude::Utc, Datelike};

mod repl;
mod syntax;
mod lexic;
mod token;

const VERSION: &str = "0.0.1";

fn get_copyright() -> String {
    let year = Utc::now().year();

    format!("Misti {}\nCopyright (c) {} Fernando Enrique Araoz Morales", VERSION, year)
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

