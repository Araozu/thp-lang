use std::io::{self, Write};

use crate::error_handling::PrintableError;
use crate::symbol_table::SymbolTable;
use crate::token::Token;

use super::codegen;
use super::lexic;
use super::semantic;
use super::syntax;

/// Executes Lexical analysis, handles errors and calls build_ast for the next phase
fn compile(input: &String, symbol_table: &mut SymbolTable) {
    let tokens = lexic::get_tokens(input);

    match tokens {
        Ok(tokens) => {
            build_ast(input, tokens, symbol_table);
        }
        Err(error) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            eprintln!("{}", error.get_error_str(&chars))
        }
    }
}

/// Executes Syntax analysis, and for now, Semantic analysis and Code generation.
///
/// Prints the generated code in stdin
fn build_ast(input: &String, tokens: Vec<Token>, symbol_table: &mut SymbolTable) {
    let ast = syntax::construct_ast(&tokens);

    match ast {
        Ok( ast) => {
            semantic::check_ast(& ast, symbol_table);

            let js_code = codegen::codegen(&ast);
            println!("{}", js_code)
        }
        Err(reason) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            eprintln!("{}", reason.get_error_str(&chars))
        }
    }
}

/// Executes the REPL, reading from stdin, compiling and emitting JS to stdout
pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut repl_symbol_table = SymbolTable::new();

    println!("REPL: Enter expressions to evaluate. Type Ctrl-D to exit.");
    loop {
        print!("> ");
        io::stdout().flush()?;
        buffer.clear();
        let read = stdin.read_line(&mut buffer);

        match read {
            Ok(0) => {
                println!("\nBye");
                break Ok(());
            }
            Ok(_) => {
                compile(&buffer, &mut repl_symbol_table);
            }
            Err(error) => {
                eprintln!("Error reading stdin.");
                break Err(error);
            }
        };
    }
}
