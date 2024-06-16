use crate::{
    lexic::token::Token,
    syntax::{ast::Expression, ParsingError, ParsingResult},
};

use super::utils::try_binary_op;

/// Parses a factor expression.
///
/// ```ebnf
/// equality = comparison, (("==" | "!="), comparison )*;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    let (comparison, next_pos) = match super::comparison::try_parse(tokens, pos) {
        Ok((expr, next_pos)) => (expr, next_pos),
        _ => return Err(ParsingError::Unmatched),
    };

    parse_many(tokens, next_pos, comparison, 0)
}

fn parse_many<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
    prev_expr: Expression<'a>,
    indentation_level: u32,
) -> ParsingResult<'a, Expression<'a>> {
    // equality = comparison, (("==" | "!="), comparison )*;
    try_binary_op(
        tokens,
        pos,
        prev_expr,
        vec!["==", "!="],
        indentation_level,
        |tokens, next_pos, prev_expr, token, indent_count: u32| match super::comparison::try_parse(
            tokens, next_pos,
        ) {
            Ok((expr, next_pos)) => {
                let expr =
                    Expression::BinaryOperator(Box::new(prev_expr), Box::new(expr), &token.value);

                parse_many(tokens, next_pos, expr, indentation_level + indent_count)
            }
            _ => return Err(ParsingError::Unmatched),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::{get_tokens, token::TokenType};

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
            }
            _ => panic!("Expected a binary operator"),
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
            }
            _ => panic!("Expected a binary operator"),
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
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_4() {
        let tokens = get_tokens(&String::from("a\n  == b\n  == c")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(next, 9);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op, "==")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_5() {
        let tokens = get_tokens(&String::from("a ==\n  b")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(next, 6);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op, "==")
            }
            _ => panic!("Expected a binary operator"),
        }
    }
}
