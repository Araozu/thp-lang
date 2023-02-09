use crate::token::{Token, TokenType};
use super::ast_types::{ValBinding, VarBinding, Binding};
use super::expression;

// Should return a 3 state value:
// - Success: binding parsed successfully
// - NotFound: the first token (var | val) was not found, so the parser should try other options
// - Error: token (var | val) was found, but then other expected tokens were not found
pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> Option<Binding> {
    let is_val = {
        let res1 = try_token_type(tokens, pos, TokenType::VAL);
        match res1 {
            Some(_) => true,
            None => {
                let res2 = try_token_type(tokens, pos, TokenType::VAR);
                match res2 {
                    Some(_) => false,
                    None => return None
                }
            }
        }
    };

    let identifier = try_token_type(tokens, pos + 1, TokenType::Identifier);
    if identifier.is_none() { return None }
    let identifier = identifier.unwrap();

    let equal_operator = try_operator(tokens, pos + 2, String::from("="));
    if equal_operator.is_none() { return None }
    let _ = equal_operator.unwrap();

    let expression = expression::try_parse(tokens, pos + 3);
    if expression.is_none() { return None }
    let expression = expression.unwrap();

    if is_val {
        Some(Binding::Val(ValBinding {
            identifier: &identifier.value,
            expression,
        }))
    }
    else {
        Some(Binding::Var(VarBinding {
            identifier: &identifier.value,
            expression,
        }))
    }
}

fn try_token_type(tokens: &Vec<Token>, pos: usize, token_type: TokenType) -> Option<&Token> {
    tokens
        .get(pos)
        .and_then(|token| {
            (token.token_type == token_type).then(|| token)
        })
}

fn try_operator(tokens: &Vec<Token>, pos: usize, operator: String) -> Option<&Token> {
    tokens
        .get(pos)
        .and_then(|token| {
            (token.token_type == TokenType::Operator && token.value == operator)
                .then(|| token)
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
            Binding::Val(binding) => {
                assert_eq!("identifier", binding.identifier);
            }
            Binding::Var(binding) => {
                assert_eq!("identifier", binding.identifier);
            }
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
}
