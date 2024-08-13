use crate::{
    lexic::token::Token,
    syntax::{ast::Expression, parseable::Parseable, ParsingError, ParsingResult},
};

use super::function_call_expr;

/// Parses an unary expression.
///
/// ```ebnf
/// unary = ("!" | "-"), expression
///       | function call expr;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    match tokens.get(pos) {
        Some(token) if token.value == "!" || token.value == "-" => {
            match Expression::try_parse(tokens, pos + 1) {
                Ok((expression, next_pos)) => Ok((
                    Expression::UnaryOperator(&token.value, Box::new(expression)),
                    next_pos,
                )),
                _ => Err(ParsingError::Unmatched),
            }
        }
        _ => function_call_expr::try_parse(tokens, pos),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn should_parse_single_expression() {
        let tokens = get_tokens(&String::from("identifier")).unwrap();
        let expression = try_parse(&tokens, 0);

        match expression {
            Ok((Expression::Identifier(value), _)) => {
                assert_eq!("identifier", format!("{}", value.value))
            }
            _ => panic!(),
        }
    }

    #[test]
    fn should_parse_unary_expression() {
        let tokens = get_tokens(&String::from("-10")).unwrap();
        let expression = try_parse(&tokens, 0);

        match expression {
            Ok((Expression::UnaryOperator(operator, expression), _)) => {
                match (operator, *expression) {
                    (op, Expression::Int(value)) => {
                        assert_eq!(*op, "-");
                        assert_eq!(value.value, "10");
                    }
                    _ => panic!("unexpected values"),
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn should_parse_grouped_unary_expression() {
        let tokens = get_tokens(&String::from("-(25 + 30)")).unwrap();
        let expression = try_parse(&tokens, 0);

        match expression {
            Ok((Expression::UnaryOperator(operator, expression), _)) => {
                assert_eq!(*operator, "-");
                match *expression {
                    Expression::BinaryOperator(_, _, _) => {
                        // :D
                    }
                    _ => panic!("unexpected values"),
                }
            }
            _ => panic!(),
        }
    }
}
