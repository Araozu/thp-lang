use super::ast::var_binding::Binding;
use super::utils::{parse_token_type, try_operator};
use super::{expression, ParsingError, ParsingResult};
use crate::error_handling::SyntaxError;
use crate::lexic::token::{Token, TokenType};

pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParsingResult<Binding> {
    let mut current_pos = pos;

    // TODO: Detect if the binding starts with a datatype
    // TODO: Revert to val/var

    /*
     * let keyword
     */
    let (is_mutable, binding_token, next_pos) = {
        match parse_token_type(tokens, current_pos, TokenType::VAL) {
            Ok((val_token, next_pos)) => (false, val_token, next_pos),
            _ => {
                // If VAL is not found, search for VAR
                match parse_token_type(tokens, current_pos, TokenType::VAR) {
                    Ok((var_token, next_pos)) => (true, var_token, next_pos),
                    _ => return Err(ParsingError::Unmatched),
                }
            }
        }
    };
    current_pos = next_pos;

    /*
     * identifier
     */
    let (identifier, next_pos) = match parse_token_type(tokens, current_pos, TokenType::Identifier)
    {
        Ok((t, n)) => (t, n),
        Err(ParsingError::Mismatch(token)) => {
            // The parser found a token, but it's not an identifier
            return Err(ParsingError::Err(SyntaxError {
                error_start: token.position,
                error_end: token.get_end_position(),
                reason: "??".into(),
            }));
        }
        Err(ParsingError::Err(error)) => {
            return Err(ParsingError::Err(error));
        }
        _ => {
            // The parser didn't find an Identifier after VAL/VAR
            return Err(ParsingError::Err(SyntaxError {
                reason: format!(
                    "There should be an identifier after a `{}` token",
                    if is_mutable { "val" } else { "var" }
                ),
                error_start: binding_token.position,
                error_end: binding_token.get_end_position(),
            }));
        }
    };
    current_pos = next_pos;

    /*
     * Equal (=) operator
     */
    let equal_operator = match try_operator(tokens, current_pos, String::from("=")) {
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
    current_pos += 1;

    let (expression, next_pos) = match expression::try_parse(tokens, current_pos) {
        Ok((exp, next)) => (exp, next),
        _ => {
            return Err(ParsingError::Err(SyntaxError {
                reason: String::from("Expected an expression after the equal `=` operator"),
                error_start: equal_operator.position,
                error_end: equal_operator.get_end_position(),
            }));
        }
    };
    current_pos = next_pos;

    let binding = Binding {
        datatype: None,
        identifier: &identifier,
        expression,
        is_mutable,
    };

    Ok((binding, current_pos))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexic::get_tokens, syntax::utils::parse_token_type};

    #[test]
    fn should_parse_val_binding() {
        let tokens = get_tokens(&String::from("val identifier = 20")).unwrap();
        let Ok((binding, _)) = try_parse(&tokens, 0) else {
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

    /*
    #[test]
    fn should_parse_binding_with_datatype() {
        let tokens = get_tokens(&String::from("Num val identifier = 20")).unwrap();
        let ParseResult::Ok(Binding::Val(binding), _) = try_parse(&tokens, 0) else {
            panic!()
        };

        assert_eq!(Some(String::from("Num")), binding.datatype);
        assert_eq!("identifier", format!("{}", binding.identifier));

        let tokens = get_tokens(&String::from("Bool var identifier = 20")).unwrap();
        let ParseResult::Ok(Binding::Var(binding), _) = try_parse(&tokens, 0) else {
            panic!()
        };

        assert_eq!(Some(String::from("Bool")), binding.datatype);
        assert_eq!("identifier", format!("{}", binding.identifier));
    }
     */

    #[test]
    fn should_return_correct_error() {
        let tokens = get_tokens(&String::from("val")).unwrap();
        assert_eq!(TokenType::VAL, tokens[0].token_type);
        assert_eq!(0, tokens[0].position);
        let binding = try_parse(&tokens, 0);

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
        let binding = try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(4, error.error_start);
                assert_eq!(7, error.error_end);
            }
            _ => panic!("Error expected"),
        }

        let tokens = get_tokens(&String::from("val \"hello\"")).unwrap();
        let binding = try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(4, error.error_start);
                assert_eq!(11, error.error_end);
            }
            _ => panic!("Error expected"),
        }
    }

    #[test]
    fn should_return_error_when_equal_op_is_wrong() {
        let tokens = get_tokens(&String::from("val id \"error\"")).unwrap();
        let binding = try_parse(&tokens, 0);

        match binding {
            Err(ParsingError::Err(error)) => {
                assert_eq!(7, error.error_start);
                assert_eq!(14, error.error_end);
            }
            _ => panic!("Error expected"),
        }
    }
}
