use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
    syntax::{ParsingError, ParsingResult},
};

use super::{
    super::{ast::FunctionDeclaration, block::parse_block, utils::parse_token_type, ParseResult},
    params_list::parse_params_list,
};

pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParsingResult<FunctionDeclaration> {
    let mut current_pos = pos;

    // `fun` keyword
    let (fun_keyword, next_pos) = match parse_token_type(tokens, current_pos, TokenType::FUN) {
        ParseResult::Ok(t, next) => (t, next),
        _ => return Err(ParsingError::Unmatched),
    };
    current_pos = next_pos;

    let (identifier, next_pos) = match parse_token_type(tokens, current_pos, TokenType::Identifier)
    {
        ParseResult::Ok(id, next) => (id, next),
        ParseResult::Err(err) => return Err(ParsingError::Err(err)),
        ParseResult::Mismatch(wrong_token) => {
            return Err(ParsingError::Err(SyntaxError {
                reason: String::from("Expected an identifier after the `fun` keyword."),
                error_start: wrong_token.position,
                error_end: wrong_token.get_end_position(),
            }));
        }
        ParseResult::Unmatched => {
            return Err(ParsingError::Err(SyntaxError {
                reason: String::from("Expected an identifier after the `fun` keyword."),
                error_start: fun_keyword.position,
                error_end: fun_keyword.get_end_position(),
            }));
        }
    };
    current_pos = next_pos;

    let (params_list, next_pos) = match parse_params_list(tokens, current_pos) {
        Ok((params, next_pos)) => (params, next_pos),
        Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
        Err(ParsingError::Mismatch(wrong_token)) => {
            return Err(ParsingError::Err(SyntaxError {
                reason: String::from("Expected an opening paren afted the function identifier."),
                error_start: wrong_token.position,
                error_end: wrong_token.get_end_position(),
            }));
        }
        Err(ParsingError::Unmatched) => {
            return Err(ParsingError::Err(SyntaxError {
                reason: String::from("Expected an opening paren afted the function identifier."),
                error_start: identifier.position,
                error_end: identifier.get_end_position(),
            }));
        }
    };
    current_pos = next_pos;

    let (block, next_pos) = match parse_block(tokens, current_pos) {
        ParseResult::Ok(block, next_pos) => (block, next_pos),
        ParseResult::Err(error) => {
            return Err(ParsingError::Err(error));
        }
        ParseResult::Mismatch(wrong_token) => {
            return Err(ParsingError::Err(SyntaxError {
                reason: String::from("Expected a block after the function declaration."),
                error_start: wrong_token.position,
                error_end: wrong_token.get_end_position(),
            }));
        }
        ParseResult::Unmatched => {
            return Err(ParsingError::Err(SyntaxError {
                reason: String::from("Expected a block after the function declaration."),
                error_start: identifier.position,
                error_end: identifier.get_end_position(),
            }));
        }
    };
    current_pos = next_pos;

    // Construct and return the function declaration
    Ok((
        FunctionDeclaration {
            identifier: &identifier,
            params_list: Box::new(params_list),
            block: Box::new(block),
        },
        current_pos,
    ))
}

#[cfg(test)]
mod tests {
    use crate::lexic::get_tokens;

    use super::*;

    #[test]
    fn should_return_none_on_wrong_initial_token() {
        let tokens = get_tokens(&String::from("val identifier = 20")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let Err(ParsingError::Unmatched) = fun_decl else {
            panic!("Expected an unmatched result: {:?}", fun_decl);
        };
    }

    #[test]
    fn should_not_parse_fun_without_identifier() {
        let tokens = get_tokens(&String::from("fun = 20")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
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
            Err(ParsingError::Err(err)) => {
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
            Err(ParsingError::Err(err)) => {
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
            Err(ParsingError::Err(err)) => {
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
            Err(ParsingError::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected a closing paren after the function identifier."
                );
                assert_eq!(err.error_start, 7);
                assert_eq!(err.error_end, 8);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id(")).unwrap();
        let fun_decl = try_parse(&tokens, 0);
        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected a closing paren after the function identifier."
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
            Err(ParsingError::Err(err)) => {
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
            Err(ParsingError::Err(err)) => {
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
            Err(ParsingError::Err(err)) => {
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
            Err(ParsingError::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected a block after the function declaration."
                );
                assert_eq!(err.error_start, 4);
                assert_eq!(err.error_end, 6);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_not_parse_fun_without_closing_brace() {
        let tokens = get_tokens(&String::from("fun id() { 20")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.reason, "Expected a closing brace after the block body.");
                assert_eq!(err.error_start, 11);
                assert_eq!(err.error_end, 13);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id() {")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
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
        let (function_declaration, _) = try_parse(&tokens, 0).unwrap();

        assert_eq!(function_declaration.identifier.value, String::from("id"));
    }
}

#[cfg(test)]
mod whitespace_test {
    use crate::lexic::get_tokens;

    use super::*;

    #[test]
    fn should_ignore_whitespace_1() {
        let tokens = get_tokens(&String::from("fun\nid() {}")).unwrap();
        let (declaration, _) = try_parse(&tokens, 0).unwrap();

        assert_eq!(declaration.identifier.value, (String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_2() {
        let tokens = get_tokens(&String::from("fun\nid\n() {}")).unwrap();
        let (declaration, _) = try_parse(&tokens, 0).unwrap();

        assert_eq!(declaration.identifier.value, (String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_3() {
        let tokens = get_tokens(&String::from("fun\nid\n(\n) {}")).unwrap();
        let (declaration, _) = try_parse(&tokens, 0).unwrap();

        assert_eq!(declaration.identifier.value, (String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_4() {
        let tokens = get_tokens(&String::from("fun id\n(\n)\n{}")).unwrap();
        let (declaration, _) = try_parse(&tokens, 0).unwrap();
        assert_eq!(declaration.identifier.value, (String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_5() {
        let tokens = get_tokens(&String::from("fun\nid() \n{\n}")).unwrap();
        let (declaration, _) = try_parse(&tokens, 0).unwrap();
        assert_eq!(declaration.identifier.value, (String::from("id")));
    }
}
