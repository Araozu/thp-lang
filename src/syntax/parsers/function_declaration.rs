use crate::{
    error_handling::{
        error_messages::SYNTAX_INVALID_FUNCTION_DECLARATION, ErrorContainer, ErrorLabel,
    },
    lexic::token::{Token, TokenType},
    syntax::{
        ast::{Block, FunctionDeclaration, Positionable},
        functions::params_list::parse_params_list,
        parseable::{Parseable, ParsingError, ParsingResult},
        utils::{parse_token_type, try_operator},
    },
};

impl<'a> Parseable<'a> for FunctionDeclaration<'a> {
    type Item = FunctionDeclaration<'a>;

    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        let mut current_pos = current_pos;

        // `fun` keyword
        let (fun_keyword, next_pos) = match parse_token_type(tokens, current_pos, TokenType::FUN) {
            Ok((t, next)) => (t, next),
            _ => return Err(ParsingError::Unmatched),
        };
        current_pos = next_pos;

        // identifier
        let (identifier, next_pos) =
            match parse_token_type(tokens, current_pos, TokenType::Identifier) {
                Ok((id, next)) => (id, next),
                Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
                Err(ParsingError::Mismatch(wrong_token)) => {
                    let label = ErrorLabel {
                        message: String::from("Expected an identifier here"),
                        start: wrong_token.position,
                        end: wrong_token.get_end_position(),
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INVALID_FUNCTION_DECLARATION,
                        error_offset: wrong_token.position,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
                }
                Err(ParsingError::Unmatched) => {
                    let label = ErrorLabel {
                        message: String::from("Expected an identifier after this `fun` keyword"),
                        start: fun_keyword.position,
                        end: fun_keyword.get_end_position(),
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INVALID_FUNCTION_DECLARATION,
                        error_offset: fun_keyword.position,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
                }
            };
        current_pos = next_pos;

        // Params list
        // TODO: impl Parseable
        let (params_list, next_pos) = match parse_params_list(tokens, current_pos) {
            Ok((params, next_pos)) => (params, next_pos),
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(wrong_token)) => {
                let label = ErrorLabel {
                    message: String::from("Expected a parameter list here"),
                    start: wrong_token.position,
                    end: wrong_token.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_FUNCTION_DECLARATION,
                    error_offset: wrong_token.get_end_position(),
                    labels: vec![label],
                    note: Some(String::from(
                        "If this function doesn't take any parameter, use an empty list `()`",
                    )),
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            Err(ParsingError::Unmatched) => {
                let label = ErrorLabel {
                    message: String::from("Expected a parameter list after this identifier"),
                    start: identifier.position,
                    end: identifier.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_FUNCTION_DECLARATION,
                    error_offset: identifier.get_end_position(),
                    labels: vec![label],
                    note: Some(String::from(
                        "If this function doesn't take any parameter, use an empty list `()`",
                    )),
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };
        current_pos = next_pos;

        // Try to parse a return type
        // TODO: abstract parsing of Datatype to parse generics
        let (return_type, next_pos) = 'return_label: {
            let (arrow_op, next_pos) = match try_operator(tokens, current_pos, "->".into()) {
                Ok((op, next)) => (op, next),
                _ => break 'return_label (None, current_pos),
            };

            // At this point the '->' operator was matched, so we expect a datatype
            match parse_token_type(tokens, next_pos, TokenType::Datatype) {
                Ok((t, next)) => (Some(t), next),
                Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
                Err(ParsingError::Mismatch(wrong_token)) => {
                    let label = ErrorLabel {
                        message: String::from("Expected a Datatype here"),
                        start: wrong_token.position,
                        end: wrong_token.get_end_position(),
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INVALID_FUNCTION_DECLARATION,
                        error_offset: wrong_token.position,
                        labels: vec![label],
                        note: Some(String::from(
                            "If you want a function without a return type, omit the arrow as well",
                        )),
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
                }
                Err(ParsingError::Unmatched) => {
                    let label = ErrorLabel {
                        message: String::from("Expected a Datatype after this arrow `->` operator"),
                        start: arrow_op.position,
                        end: arrow_op.get_end_position(),
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INVALID_FUNCTION_DECLARATION,
                        error_offset: arrow_op.position,
                        labels: vec![label],
                        note: Some(String::from(
                            "If you want a function without a return type, omit the arrow as well",
                        )),
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
                }
            }
        };
        current_pos = next_pos;

        // Function body (block)
        let (block, next_pos) = match Block::try_parse(tokens, current_pos) {
            Ok((block, next_pos)) => (block, next_pos),
            Err(ParsingError::Err(error)) => {
                return Err(ParsingError::Err(error));
            }
            Err(ParsingError::Mismatch(wrong_token)) => {
                let label = ErrorLabel {
                    message: String::from("Expected a block here, after the function declaration"),
                    start: wrong_token.position,
                    end: wrong_token.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_FUNCTION_DECLARATION,
                    error_offset: wrong_token.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            Err(ParsingError::Unmatched) => {
                let (error_start, error_end) = {
                    if let Some(return_type) = return_type {
                        (return_type.position, return_type.get_end_position())
                    } else {
                        params_list.get_position()
                    }
                };
                let label = ErrorLabel {
                    message: String::from("Expected a block here, after the function declaration"),
                    start: error_start,
                    end: error_end,
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_FUNCTION_DECLARATION,
                    error_offset: error_start,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };
        current_pos = next_pos;

        // Construct and return the function declaration
        Ok((
            FunctionDeclaration {
                identifier: &identifier,
                return_type,
                params_list: Box::new(params_list),
                block: Box::new(block),
            },
            current_pos,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexic::get_tokens;

    use super::*;

    #[test]
    fn should_return_none_on_wrong_initial_token() {
        let tokens = get_tokens(&String::from("val identifier = 20")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        let Err(ParsingError::Unmatched) = fun_decl else {
            panic!("Expected an unmatched result: {:?}", fun_decl);
        };
    }

    #[test]
    fn should_not_parse_fun_without_identifier() {
        let tokens = get_tokens(&String::from("fun = 20")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 4);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);
        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 0);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_not_parse_fun_without_parens() {
        let tokens = get_tokens(&String::from("fun id =")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 7);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);
        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 4);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_not_parse_fun_without_closing_paren() {
        let tokens = get_tokens(&String::from("fun id(=")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 7);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id(")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);
        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 6);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_not_parse_fun_when_missing_id() {
        let tokens = get_tokens(&String::from("fun")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 0);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun\n")).unwrap();
        println!("{:?}", tokens);
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 0);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_not_parse_fun_without_opening_brace() {
        let tokens = get_tokens(&String::from("fun id() =")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 9);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id()")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);
        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 4);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_not_parse_fun_without_closing_brace() {
        let tokens = get_tokens(&String::from("fun id() { ")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 9);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id() {")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 9);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_parse_simple_function_declaration() {
        let tokens = get_tokens(&String::from("fun id() {}")).unwrap();
        let (function_declaration, _) = FunctionDeclaration::try_parse(&tokens, 0).unwrap();

        assert_eq!(function_declaration.identifier.value, String::from("id"));
        assert_eq!(function_declaration.return_type, None);
    }

    #[test]
    fn should_parse_return_type() {
        let tokens = get_tokens(&String::from("fun id() -> String {}")).unwrap();
        let (function_declaration, _) = FunctionDeclaration::try_parse(&tokens, 0).unwrap();

        assert_eq!(function_declaration.identifier.value, String::from("id"));
        assert_eq!(
            function_declaration.return_type.unwrap().value,
            String::from("String")
        );
    }

    #[test]
    fn should_throw_error_on_return_type_1() {
        let tokens = get_tokens(&String::from("fun id() -> {}")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 9);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }

    #[test]
    fn should_throw_error_on_return_type_2() {
        let tokens = get_tokens(&String::from("fun id() -> ")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INVALID_FUNCTION_DECLARATION);
                assert_eq!(err.error_offset, 9);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }
    }
}

#[cfg(test)]
mod whitespace_test {
    use crate::lexic::get_tokens;

    use super::*;

    #[test]
    fn should_ignore_whitespace_1() {
        let tokens = get_tokens(&String::from("fun\nid() {}")).unwrap();
        let (declaration, _) = FunctionDeclaration::try_parse(&tokens, 0).unwrap();

        assert_eq!(declaration.identifier.value, (String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_2() {
        let tokens = get_tokens(&String::from("fun\nid\n() {}")).unwrap();
        let (declaration, _) = FunctionDeclaration::try_parse(&tokens, 0).unwrap();

        assert_eq!(declaration.identifier.value, (String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_3() {
        let tokens = get_tokens(&String::from("fun\nid\n(\n) {}")).unwrap();
        let (declaration, _) = FunctionDeclaration::try_parse(&tokens, 0).unwrap();

        assert_eq!(declaration.identifier.value, (String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_4() {
        let tokens = get_tokens(&String::from("fun id\n(\n)\n{}")).unwrap();
        let (declaration, _) = FunctionDeclaration::try_parse(&tokens, 0).unwrap();
        assert_eq!(declaration.identifier.value, (String::from("id")));
    }

    #[test]
    fn should_ignore_whitespace_5() {
        let tokens = get_tokens(&String::from("fun\nid() \n{\n}")).unwrap();
        let (declaration, _) = FunctionDeclaration::try_parse(&tokens, 0).unwrap();
        assert_eq!(declaration.identifier.value, (String::from("id")));
    }
}
