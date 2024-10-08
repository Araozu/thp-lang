use serde::Serialize;

use crate::{
    error_handling::MistiError,
    lexic::{get_tokens, token::Token},
    semantic,
    syntax::build_ast,
};
use std::io::{self, BufRead};

#[derive(Serialize)]
enum TokenizeResult {
    Ok(Vec<Token>),
    SemanticError(Vec<Token>, MistiError),
    SyntaxError(Vec<Token>, MistiError),
    LexError(MistiError),
}

pub fn tokenize_command(_options: Vec<String>) -> Result<(), ()> {
    // Get the input from stdin
    let stdin = io::stdin();

    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(reason) => {
                eprintln!("Error reading input: {}", reason);
                return Err(());
            }
        }
    }

    let input_code = lines.join("\n");
    let tokens = get_tokens(&input_code);

    let result = match tokens {
        Ok(tokens) => {
            let ast_result = build_ast(&tokens);
            match ast_result {
                Ok(ast) => match semantic::check_semantics(&ast) {
                    Ok(()) => TokenizeResult::Ok(tokens),
                    Err(error) => TokenizeResult::SemanticError(tokens, error),
                },
                Err(error) => TokenizeResult::SyntaxError(tokens, error),
            }
        }
        Err(error) => TokenizeResult::LexError(error),
    };

    let json = serde_json::to_string(&result).unwrap();
    println!("{}", json);

    Ok(())
}
