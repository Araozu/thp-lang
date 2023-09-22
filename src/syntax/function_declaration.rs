use std::thread::current;

use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
    utils::Result3,
};

use super::{
    ast::{FunctionDeclaration, TopLevelDeclaration},
    block::parse_block,
    utils::{expect_token_w, parse_token_type, try_token_type},
    ParseResult, SyntaxResult,
};

pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParseResult<FunctionDeclaration, ()> {
    let mut current_pos = pos;

    // `fun` keyword
    let fun_keyword = match try_token_type(tokens, current_pos, TokenType::FUN) {
        Result3::Ok(t) => t,
        Result3::Err(_token) => return ParseResult::Unmatched,
        Result3::None => return ParseResult::Unmatched,
    };
    current_pos += 1;

    let (identifier, next_pos) = match parse_token_type(tokens, current_pos, TokenType::Identifier)
    {
        ParseResult::Ok(id, next) => (id, next),
        ParseResult::Err(err) => return ParseResult::Err(err),
        ParseResult::Mismatch(wrong_token) => {
            return ParseResult::Err(SyntaxError {
                reason: String::from("Expected an identifier after the `fun` keyword."),
                error_start: wrong_token.position,
                error_end: wrong_token.get_end_position(),
            });
        }
        ParseResult::Unmatched => {
            return ParseResult::Err(SyntaxError {
                reason: String::from("Expected an identifier after the `fun` keyword."),
                error_start: fun_keyword.position,
                error_end: fun_keyword.get_end_position(),
            });
        }
    };
    current_pos = next_pos;

    // TODO: Call function that parses a parameter list
    let (opening_paren, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::LeftParen) {
            ParseResult::Ok(id, next) => (id, next),
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Mismatch(wrong_token) => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from(
                        "Expected an opening paren afted the function identifier.",
                    ),
                    error_start: wrong_token.position,
                    error_end: wrong_token.get_end_position(),
                });
            }
            ParseResult::Unmatched => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from(
                        "Expected an opening paren afted the function identifier.",
                    ),
                    error_start: identifier.position,
                    error_end: identifier.get_end_position(),
                });
            }
        };
    current_pos = next_pos;

    let (closing_paren, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::RightParen) {
            ParseResult::Ok(id, next) => (id, next),
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Mismatch(wrong_token) => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from("Expected a closing paren afted the function identifier."),
                    error_start: wrong_token.position,
                    error_end: wrong_token.get_end_position(),
                });
            }
            ParseResult::Unmatched => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from("Expected a closing paren afted the function identifier."),
                    error_start: opening_paren.position,
                    error_end: opening_paren.get_end_position(),
                });
            }
        };
    current_pos = next_pos;

    let (_block, next_pos) = match parse_block(tokens, current_pos) {
        ParseResult::Ok(block, next_pos) => (block, next_pos),
        ParseResult::Err(error) => {
            return ParseResult::Err(error);
        }
        ParseResult::Mismatch(wrong_token) => {
            return ParseResult::Err(SyntaxError {
                reason: String::from("Expected a block after the function declaration."),
                error_start: wrong_token.position,
                error_end: wrong_token.get_end_position(),
            });
        }
        ParseResult::Unmatched => {
            return ParseResult::Err(SyntaxError {
                reason: String::from("Expected a block after the function declaration."),
                error_start: closing_paren.position,
                error_end: closing_paren.get_end_position(),
            });
        }
    };
    current_pos = next_pos;

    // Construct and return the function declaration
    ParseResult::Ok(
        FunctionDeclaration {
            identifier: Box::new(identifier.value.clone()),
        },
        next_pos,
    )
}

#[cfg(test)]
mod tests {
    use crate::{lexic::get_tokens, syntax::ast::TopLevelDeclaration};

    use super::*;

    #[test]
    fn should_return_none_on_wrong_initial_token() {
        let tokens = get_tokens(&String::from("val identifier = 20")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let ParseResult::Unmatched = fun_decl else {
            panic!("Expected an unmatched result: {:?}", fun_decl);
        };
    }

    #[test]
    fn should_not_parse_fun_without_identifier() {
        let tokens = get_tokens(&String::from("fun = 20")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            ParseResult::Err(err) => {
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
            ParseResult::Err(err) => {
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
            ParseResult::Err(err) => {
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
            ParseResult::Err(err) => {
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
            ParseResult::Err(err) => {
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
            ParseResult::Err(err) => {
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
            ParseResult::Err(err) => {
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
            ParseResult::Err(err) => {
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
            ParseResult::Err(err) => {
                assert_eq!(
                    err.reason,
                    "Expected a block after the function declaration."
                );
                assert_eq!(err.error_start, 9);
                assert_eq!(err.error_end, 10);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id()")).unwrap();
        let fun_decl = try_parse(&tokens, 0);
        match fun_decl {
            ParseResult::Err(err) => {
                assert_eq!(
                    err.reason,
                    "Expected a block after the function declaration."
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
            ParseResult::Err(err) => {
                assert_eq!(err.reason, "Expected a closing brace after the block body.");
                assert_eq!(err.error_start, 11);
                assert_eq!(err.error_end, 13);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id() {")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            ParseResult::Err(err) => {
                assert_eq!(err.reason, "Expected a closing brace after the block body.");
                assert_eq!(err.error_start, 9);
                assert_eq!(err.error_end, 10);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_parse_simple_function_declaration() {
        let tokens = get_tokens(&String::from("fun id() {}")).unwrap();
        let ParseResult::Ok(function_declaration, _) = try_parse(&tokens, 0) else {
            panic!("Expected a function declaration.")
        };

        assert_eq!(
            function_declaration.identifier,
            Box::new(String::from("id"))
        );
    }
}

#[cfg(test)]
mod whitespace_test {
    use crate::{lexic::get_tokens, syntax::ast::TopLevelDeclaration};

    use super::*;

    #[test]
    fn should_ignore_whitespace_1() {
        let tokens = get_tokens(&String::from("fun\nid() {}")).unwrap();
        let ParseResult::Ok(declaration, _) = try_parse(&tokens, 0) else {
            panic!("Expected a function declaration.")
        };

        assert_eq!(declaration.identifier, Box::new(String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_2() {
        let tokens = get_tokens(&String::from("fun\nid\n() {}")).unwrap();
        let ParseResult::Ok(declaration, _) = try_parse(&tokens, 0) else {
            panic!("Expected a function declaration.")
        };

        assert_eq!(declaration.identifier, Box::new(String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_3() {
        let tokens = get_tokens(&String::from("fun\nid\n(\n) {}")).unwrap();
        let ParseResult::Ok(declaration, _) = try_parse(&tokens, 0) else {
            panic!("Expected a function declaration.")
        };

        assert_eq!(declaration.identifier, Box::new(String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_4() {
        let tokens = get_tokens(&String::from("fun id\n(\n)\n{}")).unwrap();
        let ParseResult::Ok(declaration, _) = try_parse(&tokens, 0) else {
            panic!("Expected a function declaration.")
        };
        assert_eq!(declaration.identifier, Box::new(String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_5() {
        let tokens = get_tokens(&String::from("fun\nid() \n{\n}")).unwrap();
        let ParseResult::Ok(declaration, _) = try_parse(&tokens, 0) else {
            panic!("Expected a function declaration.")
        };
        assert_eq!(declaration.identifier, Box::new(String::from("id")));
    }
}
