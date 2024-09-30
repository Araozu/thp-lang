use crate::error_handling::MistiError;

mod functions;
mod parseable;
mod parsers;
mod utils;

pub mod ast;

use crate::lexic::token::Token;
use ast::ModuleAST;

use self::parseable::{Parseable, ParsingError, ParsingResult};

/// Builds the Misti AST from a vector of tokens
pub fn build_ast<'a>(tokens: &'a Vec<Token>) -> Result<ModuleAST, MistiError> {
    match ModuleAST::try_parse(tokens, 0) {
        Ok((module, _)) => Ok(module),
        Err(ParsingError::Err(error)) => Err(error),
        _ => {
            // This shouldn't happen. The module parser returns an error if it finds nothing to parse.
            unreachable!("Illegal state during parsing: The Module parse should always return a result or error")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;
    use ast::ModuleMembers;
    use tests::ast::Statement;

    // TODO: Reenable when statement parsing is rewritten
    #[test]
    fn should_parse_top_level_construct_with_trailing_newline() {
        let input = String::from(" fun f1(){}\n");
        let tokens = get_tokens(&input).unwrap();
        let productions = build_ast(&tokens).unwrap().productions;

        assert_eq!(productions.len(), 1);

        match productions.get(0).unwrap() {
            ModuleMembers::Stmt(Statement::FnDecl(_f)) => {
                assert!(true)
            }
            _ => panic!("Expected a function declaration"),
        }
    }

    #[test]
    fn should_parse_2_top_level_construct() {
        let input = String::from("fun f1(){} fun f2() {}");
        let tokens = crate::lexic::get_tokens(&input).unwrap();
        let declarations = build_ast(&tokens).unwrap().productions;

        assert_eq!(declarations.len(), 2);

        match declarations.get(0).unwrap() {
            ModuleMembers::Stmt(Statement::FnDecl(_f)) => {
                assert!(true)
            }
            _ => panic!("Expected a function declaration as first production"),
        }

        match declarations.get(1).unwrap() {
            ModuleMembers::Stmt(Statement::FnDecl(_f)) => {
                assert!(true)
            }
            _ => panic!("Expected a function declaration as first production"),
        }
    }

    #[test]
    fn should_fail_on_syntax_error() {
        let input = String::from("fun gaa {}");
        let tokens = get_tokens(&input).unwrap();
        let ast = build_ast(&tokens);

        match ast {
            Ok(_) => panic!("Expected an Err"),
            Err(_) => {}
        }
    }
}
