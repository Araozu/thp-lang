use crate::{
    lexic::token::{Token, TokenType},
    syntax::{
        ast::Expression, parseable::Parseable, utils::Tokenizer, ParsingError, ParsingResult,
    },
};

/// This grammar may not be up to date. Refer to the spec for the latest grammar.
///
/// ```ebnf
/// primary = number | string | boolean | identifier | ("(", expression, ")");
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    match tokens.get_significant(pos) {
        Some((token, token_pos)) => match token.token_type {
            TokenType::Int => Ok((Expression::Int(&token.value), token_pos + 1)),
            TokenType::Float => Ok((Expression::Float(&token.value), token_pos + 1)),
            TokenType::String => Ok((Expression::String(&token.value), token_pos + 1)),
            TokenType::Identifier if token.value == "true" || token.value == "false" => {
                Ok((Expression::Boolean(token.value == "true"), token_pos + 1))
            }
            TokenType::Identifier => Ok((Expression::Identifier(&token.value), token_pos + 1)),
            TokenType::LeftParen => parse_parenthesized_expression(tokens, token_pos),
            _ => Err(ParsingError::Unmatched),
        },
        None => Err(ParsingError::Unmatched),
    }
}

fn parse_parenthesized_expression(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    let expression = Expression::try_parse(tokens, pos + 1);
    match expression {
        Ok((expression, next_pos)) => match tokens.get(next_pos) {
            Some(token) => match token.token_type {
                TokenType::RightParen => Ok((expression, next_pos + 1)),
                _ => Err(ParsingError::Unmatched),
            },
            None => Err(ParsingError::Unmatched),
        },
        _ => Err(ParsingError::Unmatched),
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
            Ok((Expression::Int(value), _)) => {
                assert_eq!("40", format!("{}", value))
            }
            _ => panic!(),
        }
    }

    #[test]
    fn should_parse_a_string() {
        let tokens = get_tokens(&String::from("\"Hello\"")).unwrap();
        let expression = try_parse(&tokens, 0);

        match expression {
            Ok((Expression::String(value), _)) => {
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
            Ok((Expression::Boolean(value), _)) => assert!(value),
            _ => panic!(),
        }
    }

    #[test]
    fn should_parse_an_identifier() {
        let tokens = get_tokens(&String::from("someIdentifier")).unwrap();
        let expression = try_parse(&tokens, 0);

        match expression {
            Ok((Expression::Identifier(value), _)) => {
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
            Ok((Expression::Identifier(value), _)) => {
                assert_eq!("identifier", format!("{}", value))
            }
            _ => panic!(),
        }
    }
}
