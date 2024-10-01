use colored::Colorize;
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
    MixedErr(Vec<Token>, MistiError),
    Err(MistiError),
}

pub fn tokenize_command(arguments: Vec<String>) -> Result<(), ()> {
    let report_level = if arguments.is_empty() {
        2
    } else {
        if arguments.len() != 2 {
            eprintln!("{}", compile_help());
            eprintln!("{}: {}", "error".on_red(), "Invalid number of arguments");
            return Err(());
        }

        if arguments[0] != "-l" {
            eprintln!("{}", compile_help());
            eprintln!("{}: {}", "error".on_red(), "Invalid command argument");
            return Err(());
        }

        let new_level = match arguments[1].parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("{}", compile_help());
                eprintln!(
                    "{}: {} {}",
                    "error".on_red(),
                    "The LEVEL argument is not a number: ",
                    arguments[1]
                );
                return Err(());
            }
        };

        if new_level < 0 || new_level > 2 {
            eprintln!("{}", compile_help());
            eprintln!("{}: {}", "error".on_red(), "LEVEL must be 0, 1 or 2");
            return Err(());
        }

        new_level
    };

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

    let tokens = match (tokens, report_level) {
        (Ok(t), 0) => {
            // If the caller requested only lexic analysis, stop here and return

            let output_value = TokenizeResult::Ok(t);
            let json = serde_json::to_string(&output_value).unwrap();
            println!("{}", json);
            return Ok(());
        }
        (Ok(t), _) => t,
        (Err(misti_error), _) => {
            let output_value = TokenizeResult::Err(misti_error);
            let json = serde_json::to_string(&output_value).unwrap();
            println!("{}", json);
            return Ok(());
        }
    };

    let ast = build_ast(&tokens);

    let ast = match (ast, report_level) {
        (Ok(_), 1) => {
            // If the caller requested only syntax analysis, stop here and return

            let output_value = TokenizeResult::Ok(tokens);
            let json = serde_json::to_string(&output_value).unwrap();
            println!("{}", json);
            return Ok(());
        }
        (Ok(a), _) => a,
        (Err(misti_error), _) => {
            let output_value = TokenizeResult::MixedErr(tokens, misti_error);
            let json = serde_json::to_string(&output_value).unwrap();
            println!("{}", json);
            return Ok(());
        }
    };

    let result = match semantic::check_semantics(&ast) {
        Ok(()) => TokenizeResult::Ok(tokens),
        Err(error) => TokenizeResult::MixedErr(tokens, error),
    };

    let json = serde_json::to_string(&result).unwrap();
    println!("{}", json);

    Ok(())
}

fn compile_help() -> String {
    format!(
        r#"Tokenize the code from stdin.

The tokenization has 3 levels:
Level 0: Perform only lexical analysis
Level 1: Performs syntactic analysis
Level 2: Performs semantic analysis

If lexical analysis fails, a lexical error is returned.
If syntax analysis fails, tokens from lexical analysis and a syntax error is returned.
If semantic analysis fails, tokens from lexical analysis and a syntax error is returned.
If the process succeedes, only tokens are returned.

Usage:

  `thp tokenize -l LEVEL`    Tokenizes THP code from stdin up to LEVEL
  `thp tokenize`             Tokenizes THP code from stdin up to level 2
        "#,
    )
}
