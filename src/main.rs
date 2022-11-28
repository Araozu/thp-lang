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

fn main() -> io::Result<()> {
    print!("{}", get_copyright());
    repl::run()
}

