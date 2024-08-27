use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
    syntax::{
        ast::{var_binding::VariableBinding, Expression},
        parseable::{Parseable, ParsingError, ParsingResult},
        utils::{parse_terminator, parse_token_type, try_operator},
    },
};

impl<'a> Parseable<'a> for VariableBinding<'a> {
    type Item = VariableBinding<'a>;

    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        /*
         * val/var keyword
         */
        let (is_var, binding_token, next_pos) = 'token: {
            // check for VAL
            if let Ok((val_token, next_pos)) = parse_token_type(tokens, current_pos, TokenType::VAL)
            {
                break 'token (false, Some(val_token), next_pos);
            };

            // check for VAR
            match parse_token_type(tokens, current_pos, TokenType::VAR) {
                Ok((var_token, next_pos)) => (true, Some(var_token), next_pos),
                // If a VAR is not found it is still possible that the binding is an implicit VAL
                _ => (false, None, current_pos),
            }
        };

        /*
         * datatype
         */
        let (datatype, next_pos) = match parse_token_type(tokens, next_pos, TokenType::Datatype) {
            Ok((t, next)) => (Some(t), next),
            _ => (None, next_pos),
        };

        // Here:
        // If the binding is None and the datatype is None, then we didn't match a binding
        if binding_token.is_none() && datatype.is_none() {
            return Err(ParsingError::Unmatched);
        }

        /*
         * identifier
         */
        let (identifier, next_pos) = match parse_token_type(tokens, next_pos, TokenType::Identifier)
        {
            Ok((t, n)) => (t, n),
            Err(ParsingError::Mismatch(token)) => {
                // The parser found a token, but it's not an identifier
                return Err(ParsingError::Err(SyntaxError {
                    error_start: token.position,
                    error_end: token.get_end_position(),
                    reason: "There should be an identifier after a binding".into(),
                }));
            }
            _ => {
                // The parser didn't find an Identifier after VAL/VAR or the Datatype
                match (binding_token, datatype) {
                    (Some(binding_token), None) => {
                        return Err(ParsingError::Err(SyntaxError {
                            reason: format!(
                                "There should be an identifier after a `{}` token",
                                if is_var { "var" } else { "val" }
                            ),
                            error_start: binding_token.position,
                            error_end: binding_token.get_end_position(),
                        }));
                    }
                    (_, Some(datatype_token)) => {
                        return Err(ParsingError::Err(SyntaxError {
                            reason: "There should be an identifier after the datatype".into(),
                            error_start: datatype_token.position,
                            error_end: datatype_token.get_end_position(),
                        }));
                    }
                    _ => {
                        unreachable!(
                            "Illegal parser state: binding_token and datatype are both None"
                        )
                    }
                };
            }
        };

        /*
         * Equal (=) operator
         */
        let equal_operator = match try_operator(tokens, next_pos, String::from("=")) {
            Ok((t, _)) => t,
            Err(ParsingError::Mismatch(t)) => {
                // The parser found a token, but it's not the `=` operator
                return Err(ParsingError::Err(SyntaxError {
                    reason: format!("There should be an equal sign `=` after the identifier"),
                    error_start: t.position,
                    error_end: t.get_end_position(),
                }));
            }
            _ => {
                // The parser didn't find the `=` operator after the identifier
                return Err(ParsingError::Err(SyntaxError {
                    reason: format!("There should be an equal sign `=` after the identifier",),
                    error_start: identifier.position,
                    error_end: identifier.get_end_position(),
                }));
            }
        };
        let next_pos = next_pos + 1;

        /*
         * Expression of the binding
         */
        let (expression, next_pos) = match Expression::try_parse(tokens, next_pos) {
            Ok((exp, next)) => (exp, next),
            _ => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected an expression after the equal `=` operator"),
                    error_start: equal_operator.position,
                    error_end: equal_operator.get_end_position(),
                }));
            }
        };

        // After the expression there should be a new line
        // to terminate the statement
        let next_pos = match parse_terminator(tokens, next_pos) {
            Ok((_, next)) => next,
            Err(ParsingError::Mismatch(t)) => {
                return Err(ParsingError::Err(SyntaxError {
                    error_start: t.position,
                    error_end: t.get_end_position(),
                    reason: format!("Unexpected token `{}`, expected a new line", t.value),
                }))
            }
            _ => unreachable!(),
        };

        let binding = VariableBinding {
            datatype,
            identifier: &identifier,
            expression,
            is_mutable: is_var,
        };

        Ok((binding, next_pos))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexic::get_tokens, syntax::utils::parse_token_type};

    #[test]
    fn should_parse_val_binding() {
        let tokens = get_tokens(&String::from("val identifier = 20")).unwrap();
        let Ok((binding, _)) = VariableBinding::try_parse(&tokens, 0) else {
            panic!()
        };

        assert_eq!("identifier", format!("{}", binding.identifier.value));
    }

    #[test]
    fn should_parse_val() {
        let tokens = get_tokens(&String::from("val")).unwrap();
        let (token, _) = parse_token_type(&tokens, 0, TokenType::VAL).unwrap();

        assert_eq!(TokenType::VAL, token.token_type);
        assert_eq!("val", token.value);
    }

    #[test]
    fn should_parse_identifier() {
        let tokens = get_tokens(&String::from("identifier")).unwrap();
        let (token, _) = parse_token_type(&tokens, 0, TokenType::Identifier).unwrap();

        assert_eq!("identifier", token.value);
    }

    #[test]
    fn should_parse_operator() {
        let tokens = get_tokens(&String::from("=")).unwrap();
        let (token, _) = try_operator(&tokens, 0, String::from("=")).unwrap();

        assert_eq!("=", token.value);
    }

    #[test]
    fn should_parse_val_binding_with_datatype() {
        let tokens = get_tokens(&String::from("val Int identifier = 20")).unwrap();
        let (binding, _) = VariableBinding::try_parse(&tokens, 0).unwrap();

        assert!(!binding.is_mutable);
        assert_eq!("Int", binding.datatype.unwrap().value);
        assert_eq!("identifier", binding.identifier.value);
    }

    #[test]
    fn should_parse_var_binding_with_datatype() {
        let tokens = get_tokens(&String::from("var Int identifier = 20")).unwrap();
        let (binding, _) = VariableBinding::try_parse(&tokens, 0).unwrap();

        assert!(binding.is_mutable);
        assert!(binding.datatype.is_some());
        assert_eq!("Int", binding.datatype.unwrap().value);
        assert_eq!("identifier", binding.identifier.value);
    }

    #[test]
    fn should_parse_implicit_val_binding() {
        let tokens = get_tokens(&String::from("Int identifier = 20")).unwrap();
        let (binding, _) = VariableBinding::try_parse(&tokens, 0).unwrap();

        assert!(!binding.is_mutable);
        assert!(binding.datatype.is_some());
        assert_eq!("Int", binding.datatype.unwrap().value);
        assert_eq!("identifier", binding.identifier.value);
    }

    #[test]
    fn should_return_error_on_implicit_val_binding() {
        let tokens = get_tokens(&String::from("Int => 20")).unwrap();
        let binding = VariableBinding::try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(4, error.error_start);
                assert_eq!(6, error.error_end);
            }
            _ => panic!("Error expected"),
        }
    }

    #[test]
    fn should_return_correct_error() {
        let tokens = get_tokens(&String::from("val")).unwrap();
        assert_eq!(TokenType::VAL, tokens[0].token_type);
        assert_eq!(0, tokens[0].position);
        let binding = VariableBinding::try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(0, error.error_start);
                assert_eq!(3, error.error_end);
            }
            _ => panic!("Error expected"),
        }
    }

    #[test]
    fn should_return_error_when_identifier_is_wrong() {
        let tokens = get_tokens(&String::from("val 322")).unwrap();
        assert_eq!(TokenType::VAL, tokens[0].token_type);
        assert_eq!(0, tokens[0].position);
        let binding = VariableBinding::try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(4, error.error_start);
                assert_eq!(7, error.error_end);
            }
            _ => panic!("Error expected"),
        }

        let tokens = get_tokens(&String::from("val \"hello\"")).unwrap();
        let binding = VariableBinding::try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(4, error.error_start);
                assert_eq!(11, error.error_end);
                assert_eq!(
                    "There should be an identifier after a binding",
                    error.reason
                );
            }
            _ => panic!("Error expected"),
        }
    }

    #[test]
    fn should_return_error_when_equal_op_is_wrong() {
        let tokens = get_tokens(&String::from("val id \"error\"")).unwrap();
        let binding = VariableBinding::try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(7, error.error_start);
                assert_eq!(14, error.error_end);
            }
            _ => panic!("Error expected"),
        }
    }

    #[test]
    fn should_return_error_when_identifier_is_empty() {
        let tokens = get_tokens(&String::from("val String ")).unwrap();
        let binding = VariableBinding::try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(4, error.error_start);
                assert_eq!(10, error.error_end);
                assert_eq!(
                    "There should be an identifier after the datatype",
                    error.reason
                );
            }
            _ => panic!("Error expected"),
        }
    }

    #[test]
    fn should_return_error_when_identifier_is_empty_2() {
        let tokens = get_tokens(&String::from("val ")).unwrap();
        let binding = VariableBinding::try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(0, error.error_start);
                assert_eq!(3, error.error_end);
                assert_eq!(
                    "There should be an identifier after a `val` token",
                    error.reason
                );
            }
            _ => panic!("Error expected"),
        }
    }

    #[test]
    fn should_error_when_equal_op_is_missing() {
        let tokens = get_tokens(&String::from("val identifier ")).unwrap();
        let binding = VariableBinding::try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(4, error.error_start);
                assert_eq!(14, error.error_end);
                assert_eq!(
                    "There should be an equal sign `=` after the identifier",
                    error.reason
                );
            }
            _ => panic!("Error expected"),
        }
    }

    #[test]
    fn should_error_when_exp_is_empty() {
        let tokens = get_tokens(&String::from("val identifier = ")).unwrap();
        let binding = VariableBinding::try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(15, error.error_start);
                assert_eq!(16, error.error_end);
                assert_eq!(
                    "Expected an expression after the equal `=` operator",
                    error.reason
                );
            }
            _ => panic!("Error expected"),
        }
    }

    #[test]
    fn should_error_when_there_is_no_delimiter() {
        let tokens = get_tokens(&String::from("val identifier = 322 print(identifier)")).unwrap();
        let binding = VariableBinding::try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(21, error.error_start);
                assert_eq!(26, error.error_end);
                assert_eq!(
                    "Unexpected token `print`, expected a new line",
                    error.reason
                );
            }
            _ => panic!("Error expected"),
        }
    }
}
