use super::ast::var_binding::{Binding, ValBinding, VarBinding};
use super::utils::{parse_token_type, try_operator};
use super::{expression, ParseResult};
use crate::error_handling::SyntaxError;
use crate::lexic::token::{Token, TokenType};
use crate::utils::Result3;

pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParseResult<Binding, ()> {
    let mut current_pos = pos;
    /*
     * val/var keyword
     */
    let (is_val, binding_token, next_pos) = {
        let res1 = parse_token_type(tokens, current_pos, TokenType::VAL);
        match res1 {
            ParseResult::Ok(val_token, next) => (true, val_token, next),
            _ => {
                let res2 = parse_token_type(tokens, current_pos, TokenType::VAR);
                match res2 {
                    ParseResult::Ok(var_token, next) => (false, var_token, next),
                    // Neither VAL nor VAR were matched, the caller should try
                    // other constructs
                    _ => return ParseResult::Unmatched,
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
        ParseResult::Ok(t, n) => (t, n),
        ParseResult::Mismatch(token) => {
            // The parser found a token, but it's not an identifier
            return ParseResult::Err(SyntaxError {
                error_start: token.position,
                error_end: token.get_end_position(),
                reason: "??".into(),
            });
        }
        ParseResult::Err(error) => {
            return ParseResult::Err(error);
        }
        _ => {
            // The parser didn't find an Identifier after VAL/VAR
            return ParseResult::Err(SyntaxError {
                reason: format!(
                    "There should be an identifier after a `{}` token",
                    if is_val { "val" } else { "var" }
                ),
                error_start: binding_token.position,
                error_end: binding_token.get_end_position(),
            });
        }
    };
    current_pos = next_pos;

    /*
     * Equal (=) operator
     */
    let equal_operator = match try_operator(tokens, current_pos, String::from("=")) {
        Result3::Ok(t) => t,
        Result3::Err(t) => {
            // The parser found a token, but it's not the `=` operator
            return ParseResult::Err(SyntaxError {
                reason: format!("There should be an equal sign `=` after the identifier"),
                error_start: t.position,
                error_end: t.get_end_position(),
            });
        }
        Result3::None => {
            // The parser didn't find the `=` operator after the identifier
            return ParseResult::Err(SyntaxError {
                reason: format!("There should be an equal sign `=` after the identifier",),
                error_start: identifier.position,
                error_end: identifier.get_end_position(),
            });
        }
    };

    let (expression, _next) = match expression::try_parse(tokens, current_pos + 1) {
        ParseResult::Ok(exp, next) => (exp, next),
        _ => {
            return ParseResult::Err(SyntaxError {
                reason: String::from("Expected an expression after the equal `=` operator"),
                error_start: equal_operator.position,
                error_end: equal_operator.get_end_position(),
            });
        }
    };

    let binding = if is_val {
        Binding::Val(ValBinding {
            datatype: None,
            identifier: Box::new(identifier.value.clone()),
            expression,
        })
    } else {
        Binding::Var(VarBinding {
            datatype: None,
            identifier: Box::new(identifier.value.clone()),
            expression,
        })
    };

    ParseResult::Ok(binding, current_pos + 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexic::get_tokens, syntax::utils::try_token_type};

    #[test]
    fn should_parse_val_binding() {
        let tokens = get_tokens(&String::from("val identifier = 20")).unwrap();
        let ParseResult::Ok(Binding::Val(binding), _) = try_parse(&tokens, 0) else {
            panic!()
        };

        assert_eq!("identifier", format!("{}", binding.identifier));
    }

    #[test]
    fn should_parse_val() {
        let tokens = get_tokens(&String::from("val")).unwrap();
        let token = *try_token_type(&tokens, 0, TokenType::VAL).unwrap();

        assert_eq!(TokenType::VAL, token.token_type);
        assert_eq!("val", token.value);
    }

    #[test]
    fn should_parse_identifier() {
        let tokens = get_tokens(&String::from("identifier")).unwrap();
        let token = *try_token_type(&tokens, 0, TokenType::Identifier).unwrap();

        assert_eq!("identifier", token.value);
    }

    #[test]
    fn should_parse_operator() {
        let tokens = get_tokens(&String::from("=")).unwrap();
        let token = *try_operator(&tokens, 0, String::from("=")).unwrap();

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
            ParseResult::Err(error) => {
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
            ParseResult::Err(error) => {
                assert_eq!(4, error.error_start);
                assert_eq!(7, error.error_end);
            }
            _ => panic!("Error expected"),
        }

        let tokens = get_tokens(&String::from("val \"hello\"")).unwrap();
        let binding = try_parse(&tokens, 0);

        match binding {
            ParseResult::Err(error) => {
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
            ParseResult::Err(error) => {
                assert_eq!(7, error.error_start);
                assert_eq!(14, error.error_end);
            }
            _ => panic!("Error expected"),
        }
    }
}
