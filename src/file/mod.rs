use colored::*;
use std::{fs, path::Path};

use crate::lexic::token::Token;
use crate::{codegen, error_handling::PrintableError, lexic, syntax};

pub fn compile_file(input: &String) -> Result<(), ()> {
    let input_path = Path::new(input);

    if !input_path.is_file() {
        eprintln!(
            "{}: {} {}",
            "error".on_red(),
            "Input path is not a valid file:".red(),
            input
        );
        return Err(());
    }

    let bytes = match fs::read(input_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("{}: Error reading input file", "error".on_red());
            eprintln!("{}", error);
            return Err(());
        }
    };

    let contents = match String::from_utf8(bytes) {
        Ok(str) => str,
        Err(error) => {
            eprintln!("{}: Input file contains invalid UTF-8", "error".on_red());
            eprintln!("{}", error);
            return Err(());
        }
    };

    let Some(out_code) = compile(&contents) else {
        return Err(());
    };

    let mut output_path = Path::new(input)
        .canonicalize()
        .expect("Invalid input path: Cannot be canonicalized");
    output_path.set_extension("php");

    match fs::write(output_path, out_code) {
        Ok(_) => Ok(()),
        Err(error) => {
            eprintln!("{}: Error writing output file", "error".on_red());
            eprintln!("{}", error);
            Err(())
        }
    }
}

/// Executes Lexical analysis, handles errors and calls build_ast for the next phase
fn compile(input: &String) -> Option<String> {
    let tokens = lexic::get_tokens(input);

    match tokens {
        Ok(tokens) => Some(build_ast(input, tokens)),
        Err(error) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            println!(
                "{}:\n{}",
                "syntax error".on_red(),
                error.get_error_str(&chars)
            );
            None
        }
    }
}

/// Executes Syntax analysis, and for now, Semantic analysis and Code generation.
///
/// Prints the generated code in stdin
fn build_ast(input: &String, tokens: Vec<Token>) -> String {
    let ast = syntax::construct_ast(&tokens);

    match ast {
        Ok(ast) => {
            match crate::semantic::check_semantics(&ast) {
                Ok(_) => {}
                Err(reason) => {
                    panic!("{}", reason)
                }
            };

            codegen::codegen(&ast)
        }
        Err(reason) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            panic!("{}", reason.get_error_str(&chars))
        }
    }
}
