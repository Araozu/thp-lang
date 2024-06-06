use crate::{
    lexic::token::{Token, TokenType},
    syntax::{ast::Expression, utils::Tokenizer, ParsingError, ParsingResult},
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

    parse_many(tokens, next_pos, unary, 0)
}

fn parse_many<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
    prev_expr: Expression<'a>,
    indentation_level: u32,
) -> ParsingResult<'a, Expression<'a>> {
    // (("/" | "*"), unary)*

    let mut indent_count: u32 = 0;

    // Handle possible indentation before binary operator
    let mut next_pos = pos;
    match (tokens.get(next_pos), tokens.get(next_pos + 1)) {
        // New indentation level
        (Some(t1), Some(t2))
            if t1.token_type == TokenType::NewLine && t2.token_type == TokenType::INDENT =>
        {
            // set indentation
            next_pos += 2;
            indent_count += 1;
        }
        // we are indented, ignore newlines
        (Some(t), _) if t.token_type == TokenType::NewLine && indentation_level > 0 => {
            next_pos += 1;
        }
        // let other handlers handle this
        _ => {}
    };

    let result = match tokens.get(next_pos) {
        Some(token) if token.value == "/" || token.value == "*" => {
            next_pos += 1;

            // Handle possible indentation after binary operator
            match (tokens.get(next_pos), tokens.get(next_pos + 1)) {
                // New indentation level
                (Some(t1), Some(t2))
                    if t1.token_type == TokenType::NewLine
                        && t2.token_type == TokenType::INDENT =>
                {
                    // set indentation
                    next_pos += 2;
                    indent_count += 1;
                }
                // we are indented, ignore newlines
                (Some(t), _) if t.token_type == TokenType::NewLine && indentation_level > 0 => {
                    next_pos += 1;
                }
                // let other handlers handle this
                _ => {}
            };

            match super::unary::try_parse(tokens, next_pos) {
                Ok((expr, next_pos)) => {
                    let expr = Expression::BinaryOperator(
                        Box::new(prev_expr),
                        Box::new(expr),
                        &token.value,
                    );

                    parse_many(tokens, next_pos, expr, indentation_level + indent_count)
                }
                _ => return Err(ParsingError::Unmatched),
            }
        }
        _ => return Ok((prev_expr, pos)),
    };

    let (new_expr, mut next_pos) = match result {
        Ok((e, n)) => (e, n),
        _ => return result,
    };

    for _ in 0..indent_count {
        // Expect a DEDENT for each indentation matched
        match tokens.get(next_pos) {
            // continue
            Some(t) if t.token_type == TokenType::DEDENT => {}
            // This should be unreachable, as the lexer always emits a DEDENT for each INDENT
            _ => unreachable!("Illegal parser state: Expected DEDENT (count: {})", indent_count),
        };

        next_pos += 1;
    }

    Ok((new_expr, next_pos))
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

    #[test]
    fn should_parse_indented_1() {
        let tokens = get_tokens(&String::from("a\n  * b")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(tokens[5].token_type, TokenType::DEDENT);
        assert_eq!(next, 6);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op, "*")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_2() {
        let tokens = get_tokens(&String::from("a\n  * b\n    * c")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(tokens[9].token_type, TokenType::DEDENT);
        assert_eq!(tokens[10].token_type, TokenType::DEDENT);
        assert_eq!(next, 11);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op, "*")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_3() {
        let tokens = get_tokens(&String::from("a\n  * b * c")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(tokens[7].token_type, TokenType::DEDENT);
        assert_eq!(next, 8);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op, "*")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_4() {
        let tokens = get_tokens(&String::from("a\n  * b\n  * c")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(next, 9);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op, "*")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_5() {
        let tokens = get_tokens(&String::from("a /\n  b")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(next, 6);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op, "/")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_6() {
        let tokens = get_tokens(&String::from("a\n  /\n    b")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(next, 9);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op, "/")
            }
            _ => panic!("Expected a binary operator"),
        }
    }
}
