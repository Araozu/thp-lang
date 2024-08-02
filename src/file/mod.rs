use colored::*;
use std::{fs, path::Path};

use crate::codegen::Transpilable;
use crate::php_ast::transformers::PHPTransformable;
use crate::{error_handling::PrintableError, lexic, syntax};

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

/// Full pipeline from THP source code to PHP output
fn compile(input: &String) -> Result<String, String> {
    //
    // Lexical analysis
    //
    let tokens = match lexic::get_tokens(input) {
        Ok(t) => t,
        Err(error) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            return Err(error.get_error_str(&chars));
        }
    };

    //
    // Syntax analysis
    //
    let ast = match syntax::build_ast(&tokens) {
        Ok(ast) => ast,
        Err(reason) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            return Err(reason.get_error_str(&chars));
        }
    };

    //
    // Semantic analysis
    //
    let res1 = crate::semantic::check_semantics(&ast);
    match res1 {
        Ok(_) => {}
        Err(reason) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            let error = format!("{}: {}", "error".on_red(), reason.get_error_str(&chars));
            return Err(error);
        }
    }

    //
    // Intermediate representation (THP -> PHP ast)
    //
    let php_ast = ast.into_php_ast();

    //
    // Codegen
    //
    Ok(php_ast.transpile())
}
