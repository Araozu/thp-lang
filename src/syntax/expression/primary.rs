use super::super::utils::Tokenizer;
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
    match tokens.get_significant(pos) {
        Some((token, token_pos)) => match token.token_type {
            TokenType::Number => ParseResult::Ok(Expression::Number(&token.value), token_pos + 1),
            TokenType::String => ParseResult::Ok(Expression::String(&token.value), token_pos + 1),
            TokenType::Identifier if token.value == "true" || token.value == "false" => {
                ParseResult::Ok(Expression::Boolean(token.value == "true"), token_pos + 1)
            }
            TokenType::Identifier => {
                ParseResult::Ok(Expression::Identifier(&token.value), token_pos + 1)
            }
            TokenType::LeftParen => parse_parenthesized_expression(tokens, token_pos),
            _ => ParseResult::Unmatched,
        },
        None => ParseResult::Unmatched,
    }
}

fn parse_parenthesized_expression(tokens: &Vec<Token>, pos: usize) -> ParseResult<Expression, ()> {
    let expression = super::try_parse(tokens, pos + 1);
    match expression {
        ParseResult::Ok(expression, next_pos) => match tokens.get(next_pos) {
            Some(token) => match token.token_type {
                TokenType::RightParen => ParseResult::Ok(expression, next_pos + 1),
                _ => ParseResult::Unmatched,
            },
            None => ParseResult::Unmatched,
        },
        _ => ParseResult::Unmatched,
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

    #[test]
    fn should_parse_grouped_expression() {
        let tokens = get_tokens(&String::from("(identifier)")).unwrap();
        let expression = try_parse(&tokens, 0);

        match expression {
            ParseResult::Ok(Expression::Identifier(value), _) => {
                assert_eq!("identifier", format!("{}", value))
            }
            _ => panic!(),
        }
    }
}
