use std::io::{self, Write};

use colored::Colorize;

use crate::codegen::Transpilable;
use crate::error_handling::PrintableError;

use super::lexic;
use super::syntax;

use crate::php_ast::transformers::PHPTransformable;

/// Executes the REPL, reading from stdin, compiling and emitting PHP to stdout
pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();

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
                compile(&buffer);
            }
            Err(error) => {
                eprintln!("Error reading stdin.");
                break Err(error);
            }
        };
    }
}

/// Full pipeline from THP source code to PHP output
fn compile(input: &String) {
    //
    // Lexical analysis
    //
    let tokens = match lexic::get_tokens(input) {
        Ok(t) => t,
        Err(error) => {
            let chars: Vec<char> = input.chars().collect();
            eprintln!("{}", error.get_error_str(&chars));
            return;
        }
    };

    //
    // Syntax analysis
    //
    let ast = match syntax::build_ast(&tokens) {
        Ok(ast) => ast,
        Err(reason) => {
            let chars: Vec<char> = input.chars().collect();
            eprintln!("{}", reason.get_error_str(&chars));
            return;
        }
    };

    //
    // Semantic analysis
    //
    let res1 = crate::semantic::check_semantics(&ast);
    match res1 {
        Ok(_) => {}
        Err(reason) => {
            let chars: Vec<char> = input.chars().collect();
            let error = format!("{}: {}", "error".on_red(), reason.get_error_str(&chars));
            eprintln!("{}", error);
            return;
        }
    }

    //
    // Intermediate representation (THP -> PHP ast)
    //
    let php_ast = ast.into_php_ast();

    //
    // Codegen
    //
    println!("{}", php_ast.transpile());
}
