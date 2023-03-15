use super::ast_types::{Binding, ValBinding, VarBinding};
use super::{expression, SyntaxResult};
use crate::error_handling::SyntaxError;
use crate::token::{Token, TokenType};

// TODO: Should return a 3 state value:
// - Success: binding parsed successfully
// - NotFound: the first token (var | val) was not found, so the parser should try other options
// - Error: token (var | val) was found, but then other expected tokens were not found
pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> Option<SyntaxResult> {
    let mut pos = pos;

    // Optional datatype annotation
    let datatype_annotation = {
        match tokens.get(pos) {
            Some(t) if t.token_type == TokenType::Datatype => {
                pos += 1;
                Some(String::from(&t.value))
            }
            // If the first token is anything else, ignore
            Some(_) => None,
            // This should never match, as there should always be at least a
            // TokenType::Semicolon or TokenType::EOF
            None => panic!(
                "Internal compiler error: Illegal token stream at src/syntax/binding.rs#try_parse"
            ),
        }
    };

    // var/val keyword
    let (is_val, binding_token) = {
        let res1 = try_token_type(tokens, pos, TokenType::VAL);
        match res1 {
            Some(val_token) => (true, val_token),
            None => {
                let res2 = try_token_type(tokens, pos, TokenType::VAR);
                match res2 {
                    Some(var_token) => (false, var_token),
                    // Neither VAL nor VAR were matched, the parser should try
                    // other constructs
                    None => return None,
                }
            }
        }
    };

    let identifier = try_token_type(tokens, pos + 1, TokenType::Identifier);
    if identifier.is_none() {
        // TODO: Differentiate between no token found and incorrect token found.
        // TODO: 
        // The parser didn't find an Identifier after VAL/VAR
        return Some(SyntaxResult::Err(SyntaxError {
            reason: format!(
                "There should be an identifier after a `{}` token", 
                if is_val {"val"} else {"var"}
            ),
            error_start: binding_token.position,
            error_end: binding_token.position + binding_token.value.len(),
        }));
    }
    let identifier = identifier.unwrap();

    let equal_operator = try_operator(tokens, pos + 2, String::from("="));
    if equal_operator.is_none() {
        // TODO: return Error
        return None;
    }

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

fn try_token_type(tokens: &Vec<Token>, pos: usize, token_type: TokenType) -> Option<&Token> {
    tokens
        .get(pos)
        .and_then(|token| (token.token_type == token_type).then(|| token))
}

fn try_operator(tokens: &Vec<Token>, pos: usize, operator: String) -> Option<&Token> {
    tokens.get(pos).and_then(|token| {
        (token.token_type == TokenType::Operator && token.value == operator).then(|| token)
    })
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
        let token = try_token_type(&tokens, 0, TokenType::VAL).unwrap();

        assert_eq!(TokenType::VAL, token.token_type);
        assert_eq!("val", token.value);
    }

    #[test]
    fn should_parse_identifier() {
        let tokens = get_tokens(&String::from("identifier")).unwrap();
        let token = try_token_type(&tokens, 0, TokenType::Identifier).unwrap();

        assert_eq!("identifier", token.value);
    }

    #[test]
    fn should_parse_operator() {
        let tokens = get_tokens(&String::from("=")).unwrap();
        let token = try_operator(&tokens, 0, String::from("=")).unwrap();

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
            _ => panic!(),
        }
    }
}
