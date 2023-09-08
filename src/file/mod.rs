use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    codegen,
    error_handling::PrintableError,
    lexic, semantic,
    symbol_table::{self, SymbolTable},
    syntax,
    token::Token,
};

pub fn compile_file(input: &String, output: &String) {
    let input_path = Path::new(input);

    if !input_path.is_file() {
        panic!("Input path is not a valid file")
    }

    let bytes = fs::read(input_path).expect("INPUT_PATH should be valid");
    let contents = String::from_utf8(bytes).expect("INPUT_PATH's encoding MUST be UTF-8");

    let js_code = compile(&contents);

    let output_path = Path::new(output);
    fs::write(output_path, js_code).expect("Error writing to output path");
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
        Ok(ast) => {
            let mut symbol_table = SymbolTable::new();
            semantic::check_ast(&ast, &mut symbol_table);

            codegen::codegen(&ast)
        }
        Err(reason) => {
            let chars: Vec<char> = input.chars().into_iter().collect();
            panic!("{}", reason.get_error_str(&chars))
        }
    }
}
