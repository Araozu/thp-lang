use crate::{
    lexic::token::{Token, TokenType},
    syntax::{ast::Expression, ParseResult},
};

/// This grammar may not be up to date. Refer to the spec for the latest grammar.
/// 
/// ```ebnf
/// primary = number | string | boolean | identifier | ("(", expression, ")");
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParseResult<Expression, ()> {
    /*
    TODO: Incorporate function_call into the grammar, figure out its precedence.
    match function_call::try_parse(tokens, pos) {
        super::ParseResult::Ok(function_call, next_pos) => {
            return ParseResult::Ok::<_, ()>(Expression::FunctionCall(function_call), next_pos)
        }
        _ => {}
    };
     */

    match tokens.get(pos) {
        Some(token) => match token.token_type {
            TokenType::Number => {
                ParseResult::Ok(Expression::Number(Box::new(token.value.clone())), pos + 1)
            }
            TokenType::String => {
                ParseResult::Ok(Expression::String(Box::new(token.value.clone())), pos + 1)
            }
            TokenType::Identifier if token.value == "true" || token.value == "false" => {
                ParseResult::Ok(Expression::Boolean(token.value == "true"), pos + 1)
            }
            TokenType::Identifier => ParseResult::Ok(
                Expression::Identifier(Box::new(token.value.clone())),
                pos + 1,
            ),
            // TODO: Parse parenthesized expressions.
            _ => ParseResult::Unmatched,
        },
        None => ParseResult::Unmatched,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn should_parse_a_number() {
        let tokens = get_tokens(&String::from("40")).unwrap();
        let expression = try_parse(&tokens, 0);

        match expression {
            ParseResult::Ok(Expression::Number(value), _) => assert_eq!("40", format!("{}", value)),
            _ => panic!(),
        }
    }

    #[test]
    fn should_parse_a_string() {
        let tokens = get_tokens(&String::from("\"Hello\"")).unwrap();
        let expression = try_parse(&tokens, 0);

        match expression {
            ParseResult::Ok(Expression::String(value), _) => {
                assert_eq!("\"Hello\"", format!("{}", value))
            }
            _ => panic!(),
        }
    }

    #[test]
    fn should_parse_a_boolean() {
        let tokens = get_tokens(&String::from("true")).unwrap();
        let expression = try_parse(&tokens, 0);

        match expression {
            ParseResult::Ok(Expression::Boolean(value), _) => assert!(value),
            _ => panic!(),
        }
    }

    #[test]
    fn should_parse_an_identifier() {
        let tokens = get_tokens(&String::from("someIdentifier")).unwrap();
        let expression = try_parse(&tokens, 0);

        match expression {
            ParseResult::Ok(Expression::Identifier(value), _) => {
                assert_eq!("someIdentifier", format!("{}", value))
            }
            _ => panic!(),
        }
    }
}
