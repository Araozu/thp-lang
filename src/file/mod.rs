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

    let out_code = match compile(&contents) {
        Ok(out_code) => out_code,
        Err(error) => {
            eprintln!("{}", error);
            return Err(());
        }
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

/// THP source code goes in, PHP code or an error comes out
fn compile(input: &String) -> Result<String, String> {
    let tokens = lexic::get_tokens(input);

    let tokens = match tokens {
        Ok(tokens) => tokens,
        Err(error) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            return Err(format!(
                "{}:\n{}",
                "syntax error".on_red(),
                error.get_error_str(&chars)
            ))
        }
    };

    build_ast(input, tokens)
}

/// Executes Syntax analysis, and for now, Semantic analysis and Code generation.
///
/// Prints the generated code in stdin
fn build_ast(input: &String, tokens: Vec<Token>) -> Result<String, String> {
    let ast = syntax::construct_ast(&tokens);

    let ast = match ast {
        Ok(ast) => ast,
        Err(reason) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            let error = format!("{}: {}", "error".on_red(), reason.get_error_str(&chars));
            return Err(error)
        }
    };

    crate::semantic::check_semantics(&ast)?;

    Ok(codegen::codegen(&ast))
}
