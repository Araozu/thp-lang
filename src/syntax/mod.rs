use crate::error_handling::{MistiError, SyntaxError};

mod binding;
mod block;
mod expression;
mod functions;
mod statement;
mod utils;

pub mod ast;

use crate::lexic::token::{Token, TokenType};
use ast::ModuleAST;

use self::ast::ModuleMembers;

pub type ParsingResult<'a, A> = Result<(A, usize), ParsingError<'a>>;

#[derive(Debug)]
pub enum ParsingError<'a> {
    /// Some other token was found than the expected one
    Mismatch(&'a Token),
    /// The parsing didn't succeed, but it's not a fatal error
    Unmatched,
    /// The parsing failed past a point of no return.
    ///
    /// For example, when parsing a function declaration
    /// the `fun` token is found, but then no identifier
    Err(SyntaxError),
}

/// Constructs the Misti AST from a vector of tokens
pub fn build_ast<'a>(tokens: &'a Vec<Token>) -> Result<ModuleAST, MistiError> {
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
            Ok((module, next_pos)) => {
                top_level_declarations.push(module);
                current_pos = next_pos;
            }
            Err(ParsingError::Err(err)) => return Err(MistiError::Syntax(err)),
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
        productions: top_level_declarations,
    })
}

fn next_construct<'a>(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<ModuleMembers> {
    // Try to parse a function declaration
    match functions::function_declaration::try_parse(tokens, current_pos) {
        Ok((declaration, next_pos)) => {
            return Ok((ModuleMembers::FunctionDeclaration(declaration), next_pos))
        }
        Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
        _ => {}
    }

    // Try to parse a binding
    match binding::try_parse(tokens, current_pos) {
        Ok((binding, next_pos)) => return Ok((ModuleMembers::Binding(binding), next_pos)),
        Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
        _ => {}
    }

    // Try to parse an expression
    match expression::try_parse(tokens, current_pos) {
        Ok((expression, next_pos)) => return Ok((ModuleMembers::Expression(expression), next_pos)),
        Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
        _ => {}
    }

    // No top level construct was found, return unmatched
    Err(ParsingError::Unmatched)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_top_level_construct_with_trailing_newline() {
        let input = String::from(" fun f1(){}\n");
        let tokens = crate::lexic::get_tokens(&input).unwrap();
        let declarations = build_ast(&tokens).unwrap().productions;

        assert_eq!(declarations.len(), 1);

        match declarations.get(0).unwrap() {
            ModuleMembers::Binding(_) => panic!("Expected a function declaration"),
            ModuleMembers::FunctionDeclaration(_f) => {
                assert!(true)
            }
            _ => panic!("Not implemented: Expression at top level"),
        }
    }

    #[test]
    fn should_parse_2_top_level_construct() {
        let input = String::from("fun f1(){} fun f2() {}");
        let tokens = crate::lexic::get_tokens(&input).unwrap();
        let declarations = build_ast(&tokens).unwrap().productions;

        assert_eq!(declarations.len(), 2);

        match declarations.get(0).unwrap() {
            ModuleMembers::Binding(_) => panic!("Expected a function declaration"),
            ModuleMembers::FunctionDeclaration(_f) => {
                assert!(true)
            }
            _ => panic!("Not implemented: Expression at top level"),
        }

        match declarations.get(1).unwrap() {
            ModuleMembers::Binding(_) => panic!("Expected a function declaration"),
            ModuleMembers::FunctionDeclaration(_f) => {
                assert!(true)
            }
            _ => panic!("Not implemented: Expression at top level"),
        }
    }
}
