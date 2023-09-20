use crate::{
    lexic::token::{Token, TokenType},
    utils::Result3,
};

use super::{
    ast::{FunctionDeclaration, TopLevelDeclaration},
    utils::{expect_token_w, try_token_type},
    SyntaxResult,
};

pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> Option<SyntaxResult> {
    let mut current_pos = pos;

    // `fun` keyword
    let fun_keyword = match try_token_type(tokens, current_pos, TokenType::FUN) {
        Result3::Ok(t) => t,
        Result3::Err(_token) => return None,
        Result3::None => return None,
    };
    current_pos += 1;

    /*

    try_token_type(
        tokens,
        current_pos,
        TokenType::Identifier,
        ignore_whitespace,
        "There should be an identifier after a `fun` token, but found `{}`",
    ) -> token, usize?

    */

    // Parse identifier
    let (identifier, next_pos) = match expect_token_w(
        tokens,
        current_pos,
        TokenType::Identifier,
        "Expected an identifier after the `fun` keyword.".into(),
        fun_keyword,
    ) {
        Ok(t) => t,
        Err(err) => return err,
    };
    current_pos = next_pos;

    let (opening_paren, next_pos) = match expect_token_w(
        tokens,
        current_pos,
        TokenType::LeftParen,
        "Expected an opening paren afted the function identifier.".into(),
        identifier,
    ) {
        Ok(t) => t,
        Err(err) => return err,
    };
    current_pos = next_pos;


    // Parse a closing paren
    let (closing_paren, next_pos) = match expect_token_w(
        tokens,
        current_pos,
        TokenType::RightParen,
        "Expected a closing paren afted the function identifier.".into(),
        opening_paren,
    ) {
        Ok(t) => t,
        Err(err) => return err,
    };
    current_pos = next_pos;

    // TODO: Replace by block parsing
    // Parse opening brace
    let (opening_brace, next_pos) = match expect_token_w(
        tokens,
        current_pos,
        TokenType::LeftBrace,
        "Expected an opening brace afted the parameter list.".into(),
        closing_paren,
    ) {
        Ok(t) => t,
        Err(err) => return err,
    };
    current_pos = next_pos;

    // Parse closing brace
    let (_closing_brace, next_pos) = match expect_token_w(
        tokens,
        current_pos,
        TokenType::RightBrace,
        "Expected a closing brace after afted the function body.".into(),
        opening_brace,
    ) {
        Ok(t) => t,
        Err(err) => return err,
    };
    current_pos = next_pos;


    // Construct and return the function declaration
    Some(SyntaxResult::Ok(
        TopLevelDeclaration::FunctionDeclaration(FunctionDeclaration {
            identifier: Box::new(identifier.value.clone()),
        }),
        current_pos,
    ))
}

#[cfg(test)]
mod tests {
    use crate::{lexic::get_tokens, syntax::ast::TopLevelDeclaration};

    use super::*;

    #[test]
    fn should_return_none_on_wrong_initial_token() {
        let tokens = get_tokens(&String::from("val identifier = 20")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        assert!(fun_decl.is_none());
    }

    #[test]
    fn should_not_parse_fun_without_identifier() {
        let tokens = get_tokens(&String::from("fun = 20")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected an identifier after the `fun` keyword."
                );
                assert_eq!(err.error_start, 4);
                assert_eq!(err.error_end, 5);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun")).unwrap();
        let fun_decl = try_parse(&tokens, 0);
        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected an identifier after the `fun` keyword."
                );
                assert_eq!(err.error_start, 0);
                assert_eq!(err.error_end, 3);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_not_parse_fun_without_parens() {
        let tokens = get_tokens(&String::from("fun id =")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected an opening paren afted the function identifier."
                );
                assert_eq!(err.error_start, 7);
                assert_eq!(err.error_end, 8);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id")).unwrap();
        let fun_decl = try_parse(&tokens, 0);
        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected an opening paren afted the function identifier."
                );
                assert_eq!(err.error_start, 4);
                assert_eq!(err.error_end, 6);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_not_parse_fun_without_closing_paren() {
        let tokens = get_tokens(&String::from("fun id(=")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected a closing paren afted the function identifier."
                );
                assert_eq!(err.error_start, 7);
                assert_eq!(err.error_end, 8);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id(")).unwrap();
        let fun_decl = try_parse(&tokens, 0);
        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected a closing paren afted the function identifier."
                );
                assert_eq!(err.error_start, 6);
                assert_eq!(err.error_end, 7);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_not_parse_fun_when_missing_id() {
        let tokens = get_tokens(&String::from("fun")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected an identifier after the `fun` keyword."
                );
                assert_eq!(err.error_start, 0);
                assert_eq!(err.error_end, 3);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun\n")).unwrap();
        println!("{:?}", tokens);
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected an identifier after the `fun` keyword."
                );
                assert_eq!(err.error_start, 0);
                assert_eq!(err.error_end, 3);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_not_parse_fun_without_opening_brace() {
        let tokens = get_tokens(&String::from("fun id() =")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected an opening brace afted the parameter list."
                );
                assert_eq!(err.error_start, 9);
                assert_eq!(err.error_end, 10);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id()")).unwrap();
        let fun_decl = try_parse(&tokens, 0);
        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected an opening brace afted the parameter list."
                );
                assert_eq!(err.error_start, 7);
                assert_eq!(err.error_end, 8);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_not_parse_fun_without_closing_brace() {
        let tokens = get_tokens(&String::from("fun id() { 20")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected a closing brace after afted the function body."
                );
                assert_eq!(err.error_start, 11);
                assert_eq!(err.error_end, 13);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id() {")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            Some(SyntaxResult::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected a closing brace after afted the function body."
                );
                assert_eq!(err.error_start, 9);
                assert_eq!(err.error_end, 10);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_parse_simple_function_declaration() {
        let tokens = get_tokens(&String::from("fun id() {}")).unwrap();
        let function_declaration = try_parse(&tokens, 0).unwrap();

        match function_declaration {
            SyntaxResult::Ok(TopLevelDeclaration::FunctionDeclaration(declaration), _) => {
                assert_eq!(declaration.identifier, Box::new(String::from("id")));
            }
            _ => panic!(
                "Expected a function declaration: {:?}",
                function_declaration
            ),
        }
    }
}

#[cfg(test)]
mod whitespace_test {
    use crate::{lexic::get_tokens, syntax::ast::TopLevelDeclaration};

    use super::*;

    #[test]
    fn should_ignore_whitespace_1() {
        let tokens = get_tokens(&String::from("fun\nid() {}")).unwrap();
        let function_declaration = try_parse(&tokens, 0).unwrap();

        match function_declaration {
            SyntaxResult::Ok(TopLevelDeclaration::FunctionDeclaration(declaration), _) => {
                assert_eq!(declaration.identifier, Box::new(String::from("id")));
            }
            _ => panic!(
                "Expected a function declaration: {:?}",
                function_declaration
            ),
        }
    }

    #[test]
    fn should_ignore_whitespace_2() {
        let tokens = get_tokens(&String::from("fun\nid\n() {}")).unwrap();
        let function_declaration = try_parse(&tokens, 0).unwrap();

        match function_declaration {
            SyntaxResult::Ok(TopLevelDeclaration::FunctionDeclaration(declaration), _) => {
                assert_eq!(declaration.identifier, Box::new(String::from("id")));
            }
            _ => panic!(
                "Expected a function declaration: {:?}",
                function_declaration
            ),
        }
    }

    #[test]
    fn should_ignore_whitespace_3() {
        let tokens = get_tokens(&String::from("fun\nid\n(\n) {}")).unwrap();
        let function_declaration = try_parse(&tokens, 0).unwrap();

        match function_declaration {
            SyntaxResult::Ok(TopLevelDeclaration::FunctionDeclaration(declaration), _) => {
                assert_eq!(declaration.identifier, Box::new(String::from("id")));
            }
            _ => panic!(
                "Expected a function declaration: {:?}",
                function_declaration
            ),
        }
    }

    #[test]
    fn should_ignore_whitespace_4() {
        let tokens = get_tokens(&String::from("fun id\n(\n)\n{}")).unwrap();
        let function_declaration = try_parse(&tokens, 0).unwrap();

        match function_declaration {
            SyntaxResult::Ok(TopLevelDeclaration::FunctionDeclaration(declaration), _) => {
                assert_eq!(declaration.identifier, Box::new(String::from("id")));
            }
            _ => panic!(
                "Expected a function declaration: {:?}",
                function_declaration
            ),
        }
    }

    #[test]
    fn should_ignore_whitespace_5() {
        let tokens = get_tokens(&String::from("fun\nid() \n{\n}")).unwrap();
        let function_declaration = try_parse(&tokens, 0).unwrap();

        match function_declaration {
            SyntaxResult::Ok(TopLevelDeclaration::FunctionDeclaration(declaration), _) => {
                assert_eq!(declaration.identifier, Box::new(String::from("id")));
            }
            _ => panic!(
                "Expected a function declaration: {:?}",
                function_declaration
            ),
        }
    }
}
