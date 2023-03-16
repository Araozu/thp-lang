use super::ast_types::{Binding, ValBinding, VarBinding};
use super::{expression, SyntaxResult};
use crate::error_handling::SyntaxError;
use crate::token::{Token, TokenType};
use crate::utils::Result3;

// TODO: Should return a 3 state value:
// - Success: binding parsed successfully
// - NotFound: the first token (var | val) was not found, so the parser should try other options
// - Error: token (var | val) was found, but then other expected tokens were not found
pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> Option<SyntaxResult> {
    let mut pos = pos;

    // Optional datatype annotation
    let datatype_annotation = {
        match try_token_type(tokens, pos, TokenType::Datatype) {
            Result3::Ok(t) => {
                pos += 1;
                Some(String::from(&t.value))
            }
            Result3::Err(_) => None,
            Result3::None => panic!(
                "Internal compiler error: Illegal token stream at src/syntax/binding.rs#try_parse"
            ),
        }
    };

    // var/val keyword
    let (is_val, binding_token) = {
        let res1 = try_token_type(tokens, pos, TokenType::VAL);
        match res1 {
            Result3::Ok(val_token) => (true, val_token),
            _ => {
                let res2 = try_token_type(tokens, pos, TokenType::VAR);
                match res2 {
                    Result3::Ok(var_token) => (false, var_token),
                    // Neither VAL nor VAR were matched, the parser should try
                    // other constructs
                    _ => return None,
                }
            }
        }
    };

    let identifier = match try_token_type(tokens, pos + 1, TokenType::Identifier) {
        Result3::Ok(t) => t,
        Result3::Err(t) => {
            // The parser found a token, but it's not an identifier
            return Some(SyntaxResult::Err(SyntaxError {
                reason: format!(
                    "There should be an identifier after a `{}` token",
                    if is_val { "val" } else { "var" }
                ),
                error_start: binding_token.position,
                error_end: binding_token.position + binding_token.value.len(),
            }));
        }
        Result3::None => {
            // TODO: Differentiate between no token found and incorrect token found.
            // The parser didn't find an Identifier after VAL/VAR
            return Some(SyntaxResult::Err(SyntaxError {
                reason: format!(
                    "There should be an identifier after a `{}` token",
                    if is_val { "val" } else { "var" }
                ),
                error_start: binding_token.position,
                error_end: binding_token.position + binding_token.value.len(),
            }));
        }
    };

    let _equal_operator: &Token = match try_operator(tokens, pos + 2, String::from("=")) {
        Result3::Ok(t) => t,
        Result3::Err(t) => {
            // TODO: Differentiate between no token found and incorrect token found.
            // The parser didn't find the `=` operator after the identifier
            return Some(SyntaxResult::Err(SyntaxError {
                reason: format!("There should be an equal sign `=` after the identifier",),
                error_start: identifier.position,
                error_end: identifier.position + identifier.value.len(),
            }));
        }
        Result3::None => {
            // TODO: Differentiate between no token found and incorrect token found.
            // The parser didn't find the `=` operator after the identifier
            return Some(SyntaxResult::Err(SyntaxError {
                reason: format!("There should be an equal sign `=` after the identifier",),
                error_start: identifier.position,
                error_end: identifier.position + identifier.value.len(),
            }));
        }
    };

    let expression = expression::try_parse(tokens, pos + 3);
    if expression.is_none() {
        // TODO: return Error
        return None;
    }
    let expression = expression.unwrap();

    let binding = if is_val {
        Binding::Val(ValBinding {
            datatype: datatype_annotation,
            identifier: &identifier.value,
            expression,
        })
    } else {
        Binding::Var(VarBinding {
            datatype: datatype_annotation,
            identifier: &identifier.value,
            expression,
        })
    };

    Some(SyntaxResult::Ok(binding))
}

/// Expects the token at `pos` to be of type `token_type`
fn try_token_type(tokens: &Vec<Token>, pos: usize, token_type: TokenType) -> Result3<&Token> {
    match tokens.get(pos) {
        Some(t) if t.token_type == token_type => Result3::Ok(t),
        Some(t) => Result3::Err(t),
        None => Result3::None,
    }
}

fn try_operator(tokens: &Vec<Token>, pos: usize, operator: String) -> Result3<&Token> {
    match tokens.get(pos) {
        Some(t) if t.token_type == TokenType::Operator && t.value == operator => Result3::Ok(t),
        Some(t) => Result3::Err(t),
        None => Result3::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn should_parse_val_binding() {
        let tokens = get_tokens(&String::from("val identifier = 20")).unwrap();
        let binding = try_parse(&tokens, 0).unwrap();

        match binding {
            SyntaxResult::Ok(Binding::Val(binding)) => {
                assert_eq!("identifier", binding.identifier);
            }
            _ => panic!(),
        }
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

    #[test]
    fn should_parse_binding_with_datatype() {
        let tokens = get_tokens(&String::from("Num val identifier = 20")).unwrap();
        let binding = try_parse(&tokens, 0).unwrap();

        match binding {
            SyntaxResult::Ok(Binding::Val(binding)) => {
                assert_eq!(Some(String::from("Num")), binding.datatype);
                assert_eq!("identifier", binding.identifier);
            }
            _ => panic!(),
        }

        let tokens = get_tokens(&String::from("Bool var identifier = true")).unwrap();
        let binding = try_parse(&tokens, 0).unwrap();

        match binding {
            SyntaxResult::Ok(Binding::Var(binding)) => {
                assert_eq!(Some(String::from("Bool")), binding.datatype);
                assert_eq!("identifier", binding.identifier);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn should_return_correct_error() {
        let tokens = get_tokens(&String::from("val")).unwrap();
        assert_eq!(TokenType::VAL, tokens[0].token_type);
        assert_eq!(0, tokens[0].position);
        let binding = try_parse(&tokens, 0).unwrap();

        match binding {
            SyntaxResult::Err(error) => {
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
        let binding = try_parse(&tokens, 0).unwrap();

        match binding {
            SyntaxResult::Err(error) => {
                // assert_eq!(4, error.error_start);
                // assert_eq!(7, error.error_end);
            }
            _ => panic!("Error expected")
        }
    }
}
