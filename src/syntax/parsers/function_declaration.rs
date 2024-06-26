use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
    syntax::{
        ast::{Block, FunctionDeclaration},
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
                    return Err(ParsingError::Err(SyntaxError {
                        reason: String::from("Expected an identifier after the `fun` keyword."),
                        error_start: wrong_token.position,
                        error_end: wrong_token.get_end_position(),
                    }));
                }
                Err(ParsingError::Unmatched) => {
                    return Err(ParsingError::Err(SyntaxError {
                        reason: String::from("Expected an identifier after the `fun` keyword."),
                        error_start: fun_keyword.position,
                        error_end: fun_keyword.get_end_position(),
                    }));
                }
            };
        current_pos = next_pos;

        // Params list
        // TODO: impl Parseable
        let (params_list, next_pos) = match parse_params_list(tokens, current_pos) {
            Ok((params, next_pos)) => (params, next_pos),
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(wrong_token)) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from(
                        "Expected an opening paren after the function identifier.",
                    ),
                    error_start: wrong_token.position,
                    error_end: wrong_token.get_end_position(),
                }));
            }
            Err(ParsingError::Unmatched) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from(
                        "Expected an opening paren after the function identifier.",
                    ),
                    error_start: identifier.position,
                    error_end: identifier.get_end_position(),
                }));
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
                    return Err(ParsingError::Err(SyntaxError {
                        reason: String::from("Expected a datatype after the arrow operator."),
                        error_start: wrong_token.position,
                        error_end: wrong_token.get_end_position(),
                    }));
                }
                Err(ParsingError::Unmatched) => {
                    return Err(ParsingError::Err(SyntaxError {
                        reason: String::from("Expected a datatype after the arrow operator."),
                        error_start: arrow_op.position,
                        error_end: arrow_op.get_end_position(),
                    }));
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
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected a block after the function declaration."),
                    error_start: wrong_token.position,
                    error_end: wrong_token.get_end_position(),
                }));
            }
            Err(ParsingError::Unmatched) => {
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
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);
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
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected an opening paren after the function identifier."
                );
                assert_eq!(err.error_start, 7);
                assert_eq!(err.error_end, 8);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);
        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(
                    err.reason,
                    "Expected an opening paren after the function identifier."
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
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

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
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);
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
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

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
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

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
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

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
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);
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
        let tokens = get_tokens(&String::from("fun id() { ")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

        match fun_decl {
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.reason, "Expected a closing brace after the block body.");
                assert_eq!(err.error_start, 9);
                assert_eq!(err.error_end, 10);
            }
            _ => panic!("Expected an error: {:?}", fun_decl),
        }

        let tokens = get_tokens(&String::from("fun id() {")).unwrap();
        let fun_decl = FunctionDeclaration::try_parse(&tokens, 0);

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
                assert_eq!(err.reason, "Expected a datatype after the arrow operator.");
                assert_eq!(err.error_start, 12);
                assert_eq!(err.error_end, 13);
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
                assert_eq!(err.reason, "Expected a datatype after the arrow operator.");
                assert_eq!(err.error_start, 9);
                assert_eq!(err.error_end, 11);
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
