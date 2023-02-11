use std::io::{self, Write};

use crate::symbol_table::SymbolTable;
use crate::token::Token;

use super::lexic;
use super::syntax;
use super::semantic;
use super::codegen;

/// Executes Lexical analysis, handles errors and calls build_ast for the next phase
fn compile(input: &String) {
    let _tokens = lexic::get_tokens(input);

    match _tokens {
        Ok(tokens) => {
            build_ast(tokens);
        },
        Err(error) => {
            eprintln!("Error scanning.\n{} at pos {}", error.reason, error.position)
        }
    }

}

/// Executes Syntax analysis, and for now, Semantic analysis and Code generation.
///
/// Prints the generated code in stdin
fn build_ast(tokens: Vec<Token>) {
    let ast = syntax::construct_ast(&tokens);

    match ast {
        Ok(mut ast) => {
            let mut table = SymbolTable::new();
            semantic::check_ast(&mut ast, &mut table);
            let js_code = codegen::codegen(&ast);
            println!("{}", js_code)
        }
        Err(reason) => {
            eprintln!("Syntax error.\n{}", reason)
        }
    }
}

/// Executes the REPL, reading from stdin, compiling and emitting JS to stdout
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
