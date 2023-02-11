use crate::token::{Token, TokenType};
use super::ast_types::Expression;

/// An expression can be:
///
/// - A number
/// - A string
/// - A boolean
/// - An identifier
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> Option<Expression> {
    tokens
        .get(pos)
        .and_then(|token| {
            match token.token_type {
                TokenType::Number => {
                    Some(Expression::Number(&token.value))
                }
                TokenType::String => {
                    Some(Expression::String(&token.value))
                }
                TokenType::Identifier if token.value == "true" || token.value == "false" => {
                    Some(Expression::Boolean(token.value == "true"))
                }
                TokenType::Identifier => {
                    Some(Expression::Identifier(&token.value))
                }
                _ => None
            }
        })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn should_parse_a_number() {
        let tokens = get_tokens(&String::from("40")).unwrap();
        let expression = try_parse(&tokens, 0).unwrap();

        match expression {
            Expression::Number(value) => assert_eq!("40", value),
            _ => panic!() 
        }
    }
    
    #[test]
    fn should_parse_a_string() {
        let tokens = get_tokens(&String::from("\"Hello\"")).unwrap();
        let expression = try_parse(&tokens, 0).unwrap();

        match expression {
            Expression::String(value) => assert_eq!("Hello", value),
            _ => panic!() 
        }
    }
    
    #[test]
    fn should_parse_a_boolean() {
        let tokens = get_tokens(&String::from("true")).unwrap();
        let expression = try_parse(&tokens, 0).unwrap();

        match expression {
            Expression::Boolean(value) => assert!(value),
            _ => panic!() 
        }
    }
    
    #[test]
    fn should_parse_an_identifier() {
        let tokens = get_tokens(&String::from("someIdentifier")).unwrap();
        let expression = try_parse(&tokens, 0).unwrap();

        match expression {
            Expression::Identifier(value) => assert_eq!("someIdentifier", value),
            _ => panic!() 
        }
    }
}
