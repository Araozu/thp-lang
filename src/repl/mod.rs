use std::io::{self, Write};

use crate::symbol_table::SymbolTable;

use super::lexic;
use super::syntax;
use super::semantic;

fn compile(input: &String) {
    let _tokens = lexic::get_tokens(input);

    match _tokens {
        Ok(tokens) => {
            let mut ast = syntax::construct_ast(&tokens).unwrap();
            let mut table = SymbolTable::new();
            semantic::check_ast(&mut ast, &mut table);
        },
        Err(error) => {
            eprintln!("Error scanning.\n{} at pos {}", error.reason, error.position)
        }
    }

}

pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    println!("REPL: Enter expressions to evaluate. Type Ctrl-D to exit.");
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        buffer.clear();
        let read = stdin.read_line(&mut buffer);

        match read {
            Ok(0) => {
                println!("\nBye");
                break Ok(())
            },
            Ok(_) => {
                compile(&buffer);
            },
            Err(error) => {
                eprintln!("Error reading stdin.");
                break Err(error)
            }
        };
    }
}
