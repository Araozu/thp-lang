use std::{fs, path::Path};

use crate::lexic::token::Token;
use crate::{codegen, error_handling::PrintableError, lexic, syntax};

pub fn compile_file(input: &String) {
    let input_path = Path::new(input);

    if !input_path.is_file() {
        panic!("Input path is not a valid file")
    }

    let bytes = fs::read(input_path).expect("INPUT_PATH should be valid");
    let contents = String::from_utf8(bytes).expect("INPUT_PATH's encoding MUST be UTF-8");

    let out_code = compile(&contents);

    let mut output_path = Path::new(input).canonicalize().unwrap();
    output_path.set_extension("php");
    fs::write(output_path, out_code).expect("Error writing to output path");
}

/// Executes Lexical analysis, handles errors and calls build_ast for the next phase
fn compile(input: &String) -> String {
    let tokens = lexic::get_tokens(input);

    match tokens {
        Ok(tokens) => build_ast(input, tokens),
        Err(error) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            panic!("{}", error.get_error_str(&chars))
        }
    }
}

/// Executes Syntax analysis, and for now, Semantic analysis and Code generation.
///
/// Prints the generated code in stdin
fn build_ast(input: &String, tokens: Vec<Token>) -> String {
    let ast = syntax::construct_ast(&tokens);

    match ast {
        Ok(ast) => codegen::codegen(&ast),
        Err(reason) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            panic!("{}", reason.get_error_str(&chars))
        }
    }
}
