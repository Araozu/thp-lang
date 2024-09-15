use crate::{
    error_handling::{
        error_messages::{SYNTAX_INCOMPLETE_STATEMENT, SYNTAX_INVALID_VARIABLE_DECLARATION},
        ErrorContainer, ErrorLabel,
    },
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
                let label = ErrorLabel {
                    message: String::from("Expected an identifier here"),
                    start: token.position,
                    end: token.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_VARIABLE_DECLARATION,
                    error_offset: token.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            _ => {
                // The parser didn't find an Identifier after VAL/VAR or the Datatype
                match (binding_token, datatype) {
                    (Some(binding_token), None) => {
                        let label = ErrorLabel {
                            message: format!(
                                "There should be an identifier after this `{}` token",
                                if is_var { "var" } else { "val" }
                            ),
                            start: binding_token.position,
                            end: binding_token.get_end_position(),
                        };
                        let econtainer = ErrorContainer {
                            error_code: SYNTAX_INVALID_VARIABLE_DECLARATION,
                            error_offset: binding_token.position,
                            labels: vec![label],
                            note: None,
                            help: None,
                        };
                        return Err(ParsingError::Err(econtainer));
                    }
                    (_, Some(datatype_token)) => {
                        let label = ErrorLabel {
                            message: String::from(
                                "There should be an identifier after this datatype",
                            ),
                            start: datatype_token.position,
                            end: datatype_token.get_end_position(),
                        };
                        let econtainer = ErrorContainer {
                            error_code: SYNTAX_INVALID_VARIABLE_DECLARATION,
                            error_offset: datatype_token.position,
                            labels: vec![label],
                            note: None,
                            help: None,
                        };
                        return Err(ParsingError::Err(econtainer));
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
                let label = ErrorLabel {
                    message: String::from(
                        "Expected an equal sign `=` here, after the variable identifier",
                    ),
                    start: t.position,
                    end: t.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_VARIABLE_DECLARATION,
                    error_offset: t.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            _ => {
                // The parser didn't find the `=` operator after the identifier
                let label = ErrorLabel {
                    message: String::from("Expected an equal sign `=` after this identifier"),
                    start: identifier.position,
                    end: identifier.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_VARIABLE_DECLARATION,
                    error_offset: identifier.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };
        let next_pos = next_pos + 1;

        /*
         * Expression of the binding
         */
        let (expression, next_pos) = match Expression::try_parse(tokens, next_pos) {
            Ok((exp, next)) => (exp, next),
            _ => {
                let label = ErrorLabel {
                    message: String::from("Expected an expression after this equal `=` operator"),
                    start: equal_operator.position,
                    end: equal_operator.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_VARIABLE_DECLARATION,
                    error_offset: equal_operator.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };

        // After the expression there should be a new line
        // to terminate the statement
        let next_pos = match parse_terminator(tokens, next_pos) {
            Ok((_, next)) => next,
            Err(ParsingError::Mismatch(t)) => {
                let label = ErrorLabel {
                    message: String::from("Expected a new line here, found another token"),
                    start: t.position,
                    end: t.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INCOMPLETE_STATEMENT,
                    error_offset: t.position,
                    labels: vec![label],
                    note: Some(String::from("There may only be one statement per line")),
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
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
                assert_eq!(4, error.error_offset);
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
                assert_eq!(0, error.error_offset);
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
                assert_eq!(4, error.error_offset);
            }
            _ => panic!("Error expected"),
        }

        let tokens = get_tokens(&String::from("val \"hello\"")).unwrap();
        let binding = VariableBinding::try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(4, error.error_offset);
                assert_eq!(error.error_code, SYNTAX_INVALID_VARIABLE_DECLARATION)
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
                assert_eq!(7, error.error_offset);
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
                assert_eq!(4, error.error_offset);
                assert_eq!(error.error_code, SYNTAX_INVALID_VARIABLE_DECLARATION)
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
                assert_eq!(0, error.error_offset);
                assert_eq!(error.error_code, SYNTAX_INVALID_VARIABLE_DECLARATION)
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
                assert_eq!(4, error.error_offset);
                assert_eq!(error.error_code, SYNTAX_INVALID_VARIABLE_DECLARATION)
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
                assert_eq!(15, error.error_offset);
                assert_eq!(error.error_code, SYNTAX_INVALID_VARIABLE_DECLARATION)
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
                assert_eq!(21, error.error_offset);
                assert_eq!(error.error_code, SYNTAX_INCOMPLETE_STATEMENT)
            }
            _ => panic!("Error expected"),
        }
    }
}
