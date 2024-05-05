use crate::{
    lexic::token::Token,
    syntax::{ast::Expression, ParsingError, ParsingResult},
};

/// Parses a factor expression.
///
/// ```ebnf
/// factor = unary, (("/" | "*"), unary)*;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    let (unary, next_pos) = match super::unary::try_parse(tokens, pos) {
        Ok((expr, next_pos)) => (expr, next_pos),
        _ => return Err(ParsingError::Unmatched),
    };

    parse_many(tokens, next_pos, unary)
}

fn parse_many<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
    prev_expr: Expression<'a>,
) -> ParsingResult<'a, Expression<'a>> {
    // (("/" | "*"), unary)*

    match tokens.get(pos) {
        Some(token) if token.value == "/" || token.value == "*" => {
            match super::unary::try_parse(tokens, pos + 1) {
                Ok((expr, next_pos)) => {
                    let expr = Expression::BinaryOperator(
                        Box::new(prev_expr),
                        Box::new(expr),
                        &token.value,
                    );

                    parse_many(tokens, next_pos, expr)
                }
                _ => Err(ParsingError::Unmatched),
            }
        }
        _ => Ok((prev_expr, pos)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn should_parse_comparison() {
        let tokens = get_tokens(&String::from("a * b")).unwrap();
        let result = try_parse(&tokens, 0);

        match result {
            Ok((expr, _)) => match expr {
                Expression::BinaryOperator(exp1, exp2, op) => {
                    match (*exp1, *exp2) {
                        (Expression::Identifier(id1), Expression::Identifier(id2)) => {
                            assert_eq!("a", id1);
                            assert_eq!("b", id2);
                        }
                        _ => panic!("Expected 2 identifiers"),
                    }
                    assert_eq!("*", op)
                }
                _ => panic!("Expected a binary expression with 2 identifiers"),
            },
            Err(err) => {
                panic!("{:?}", err)
            }
        }
    }

    #[test]
    fn should_not_parse_unfinished_comparison() {
        let tokens = get_tokens(&String::from("a /")).unwrap();
        let result = try_parse(&tokens, 0);

        match result {
            Err(ParsingError::Unmatched) => assert!(true),
            _ => panic!("Expected an Unmatched error"),
        }
    }
}

