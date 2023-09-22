use std::f32::consts::E;

use crate::error_handling::{MistiError, SyntaxError};

mod binding;
mod block;
mod expression;
mod function_declaration;
mod utils;

pub mod ast;

use crate::lexic::token::{Token, TokenType};
use ast::ModuleAST;

use self::ast::TopLevelDeclaration;

#[derive(Debug)]
pub enum SyntaxResult {
    ///
    /// A construct has been found
    Ok(TopLevelDeclaration, usize),
    ///
    /// No construct was found
    None,
    ///
    /// A construct was found, but there was an error parsing it
    Err(SyntaxError),
}

#[derive(Debug)]
pub enum ParseResult<A, B> {
    Ok(A, usize),
    Err(SyntaxError),
    Mismatch(B),
    Unmatched,
}

/// Constructs the Misti AST from a vector of tokens
pub fn construct_ast<'a>(tokens: &'a Vec<Token>) -> Result<ModuleAST, MistiError> {
    let mut top_level_declarations = Vec::new();
    let token_amount = tokens.len();
    let mut current_pos = 0;

    // Minus one because the last token is always EOF
    while current_pos < token_amount - 1 {
        // Ignore newlines
        if tokens[current_pos].token_type == TokenType::NewLine {
            current_pos += 1;
            continue;
        }

        match next_construct(tokens, current_pos) {
            ParseResult::Ok(module, next_pos) => {
                top_level_declarations.push(module);
                current_pos = next_pos;
            }
            ParseResult::Err(err) => return Err(MistiError::Syntax(err)),
            _ => {
                return Err(MistiError::Syntax(SyntaxError {
                    reason: String::from("PARSER couldn't parse any construction"),
                    // FIXME: This should get the position of the _token_ that current_pos points to
                    error_start: current_pos,
                    error_end: current_pos,
                }));
            }
        }
    }

    Ok(ModuleAST {
        declarations: top_level_declarations,
    })
}

fn next_construct<'a>(
    tokens: &'a Vec<Token>,
    current_pos: usize,
) -> ParseResult<TopLevelDeclaration, ()> {
    None.or_else(
        || match function_declaration::try_parse(tokens, current_pos) {
            ParseResult::Ok(declaration, next_pos) => Some(ParseResult::Ok(
                TopLevelDeclaration::FunctionDeclaration(declaration),
                next_pos,
            )),
            ParseResult::Err(err) => Some(ParseResult::Err(err)),
            _ => None,
        },
    )
    .unwrap_or_else(|| ParseResult::Unmatched)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_top_level_construct_with_trailing_newline() {
        let input = String::from("fun f1(){}\n");
        let tokens = crate::lexic::get_tokens(&input).unwrap();
        let declarations = construct_ast(&tokens).unwrap().declarations;

        assert_eq!(declarations.len(), 1);

        match declarations.get(0).unwrap() {
            TopLevelDeclaration::Binding(_) => panic!("Expected a function declaration"),
            TopLevelDeclaration::FunctionDeclaration(_f) => {
                assert!(true)
            }
        }
    }

    #[test]
    fn should_parse_2_top_level_construct() {
        let input = String::from("fun f1(){} fun f2() {}");
        let tokens = crate::lexic::get_tokens(&input).unwrap();
        let declarations = construct_ast(&tokens).unwrap().declarations;

        assert_eq!(declarations.len(), 2);

        match declarations.get(0).unwrap() {
            TopLevelDeclaration::Binding(_) => panic!("Expected a function declaration"),
            TopLevelDeclaration::FunctionDeclaration(_f) => {
                assert!(true)
            }
        }

        match declarations.get(1).unwrap() {
            TopLevelDeclaration::Binding(_) => panic!("Expected a function declaration"),
            TopLevelDeclaration::FunctionDeclaration(_f) => {
                assert!(true)
            }
        }
    }
}
