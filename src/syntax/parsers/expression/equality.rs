use crate::{
    lexic::token::{Token, TokenType},
    syntax::{ast::Expression, ParsingError, ParsingResult},
};

/// Parses a factor expression.
///
/// ```ebnf
/// equality = comparison, (("==" | "!="), comparison )*;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    // TODO: This must be newline/indentation aware
    let (comparison, next_pos) = match super::comparison::try_parse(tokens, pos) {
        Ok((expr, next_pos)) => (expr, next_pos),
        _ => return Err(ParsingError::Unmatched),
    };

    parse_many(tokens, next_pos, comparison)
}

fn parse_many<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
    prev_expr: Expression<'a>,
) -> ParsingResult<'a, Expression<'a>> {
    // equality = comparison, (("==" | "!="), comparison )*;

    match tokens.get(pos) {
        Some(token) if token.value == "==" || token.value == "!=" => {
            match super::comparison::try_parse(tokens, pos + 1) {
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
        // If token is a newline: check if the following token is INDENT.
        // If so, ignore those 2 and continue parsing
        // Then, we should find a DEDENT token to finish this expression?
        Some(token) if token.token_type == TokenType::NewLine => {
            match tokens.get(pos + 1) {
                Some(t) if t.token_type == TokenType::INDENT => {
                    // Ignore indentation and continue parsing
                    let result = parse_many(tokens, pos + 2, prev_expr);
                    // Expect a DEDENT token
                    match result {
                        Ok((expr, next)) => {
                            match tokens.get(next) {
                                Some(t) if t.token_type == TokenType::DEDENT => {
                                    Ok((expr, next + 1))
                                }
                                _ => unreachable!("Invalid parser state: expected a DEDENT after parsing an indented expression")
                            }
                        }
                        _ => result
                    }
                }
                _ => {
                    // Return current parsed value
                    return Ok((prev_expr, pos));
                }
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
        let tokens = get_tokens(&String::from("a == b")).unwrap();
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
                    assert_eq!("==", op)
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
        let tokens = get_tokens(&String::from("a ==")).unwrap();
        let result = try_parse(&tokens, 0);

        match result {
            Err(ParsingError::Unmatched) => assert!(true),
            _ => panic!("Expected an Unmatched error"),
        }
    }

    #[test]
    fn should_parse_indented_1() {
        let tokens = get_tokens(&String::from("a\n  == b")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(tokens[5].token_type, TokenType::DEDENT);
        assert_eq!(next, 6);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op, "==")
            },
            _ => panic!("Expected a binary operator")
        }
    }

    #[test]
    fn should_parse_indented_2() {
        let tokens = get_tokens(&String::from("a\n  == b\n    == c")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(tokens[9].token_type, TokenType::DEDENT);
        assert_eq!(tokens[10].token_type, TokenType::DEDENT);
        assert_eq!(next, 11);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op, "==")
            },
            _ => panic!("Expected a binary operator")
        }
    }

    #[test]
    fn should_parse_indented_3() {
        let tokens = get_tokens(&String::from("a\n  == b == c")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(tokens[7].token_type, TokenType::DEDENT);
        assert_eq!(next, 8);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op, "==")
            },
            _ => panic!("Expected a binary operator")
        }
    }

}
