use crate::{
    lexic::token::Token,
    syntax::{ast::Expression, ParsingError, ParsingResult},
};

use super::utils::parse_many;

/// Parses a factor expression.
///
/// ```ebnf
/// term = factor, (("-" | "+" | "++"), factor)*;
/// ```
pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<Expression> {
    let (factor, next_pos) = match super::factor::try_parse(tokens, pos) {
        Ok((expr, next_pos)) => (expr, next_pos),
        _ => return Err(ParsingError::Unmatched),
    };

    parse_many(tokens, next_pos, factor, 0, &vec!["-", "+", "++"])
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;
    use crate::lexic::get_tokens;
    use crate::lexic::token::TokenType;

    #[test]
    fn should_parse_comparison() {
        let tokens = get_tokens(&String::from("a + b")).unwrap();
        let result = try_parse(&tokens, 0);

        match result {
            Ok((expr, _)) => match expr {
                Expression::BinaryOperator(exp1, exp2, op) => {
                    match (*exp1, *exp2) {
                        (Expression::Identifier(id1), Expression::Identifier(id2)) => {
                            assert_eq!("a", id1.value);
                            assert_eq!("b", id2.value);
                        }
                        _ => panic!("Expected 2 identifiers"),
                    }
                    assert_eq!("+", op.value)
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
        let tokens = get_tokens(&String::from("a -")).unwrap();
        let result = try_parse(&tokens, 0);

        match result {
            Err(ParsingError::Unmatched) => assert!(true),
            _ => panic!("Expected an Unmatched error"),
        }
    }

    #[test]
    fn should_parse_indented_1() {
        let tokens = get_tokens(&String::from("a\n  + b")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(tokens[4].token_type, TokenType::DEDENT);
        assert_eq!(next, 5);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op.value, "+")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_2() {
        let tokens = get_tokens(&String::from("a\n  + b\n    + c")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();
        assert_eq!(next, 9);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op.value, "+")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_3() {
        let tokens = get_tokens(&String::from("a\n  + b + c")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(tokens[6].token_type, TokenType::DEDENT);
        assert_eq!(next, 7);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op.value, "+")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_4() {
        let tokens = get_tokens(&String::from("a\n  + b\n  + c")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(next, 8);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op.value, "+")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_5() {
        let tokens = get_tokens(&String::from("a +\n  b")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(next, 5);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op.value, "+")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_indented_6() {
        let tokens = get_tokens(&String::from("a\n  + b\nc")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();

        assert_eq!(next, 5);

        match result {
            Expression::BinaryOperator(_, _, op) => {
                assert_eq!(op.value, "+")
            }
            _ => panic!("Expected a binary operator"),
        }
    }

    #[test]
    fn should_parse_correct_precedence() {
        let tokens = get_tokens(&String::from("1 + 2 * 3")).unwrap();
        let (result, next) = try_parse(&tokens, 0).unwrap();
        assert_eq!(next, 5);
        match result {
            Expression::BinaryOperator(lexpr, rexpr, op) => {
                assert_eq!(op.value, "+");

                match (*lexpr, *rexpr) {
                    (Expression::Int(lvalue), Expression::BinaryOperator(llexpr, rrexpr, oop)) => {
                        assert_eq!(oop.value, "*");
                        assert_eq!(lvalue.value, "1");

                        match (*llexpr, *rrexpr) {
                            (Expression::Int(left), Expression::Int(right)) => {
                                assert_eq!(left.value, "2");
                                assert_eq!(right.value, "3");
                            }
                            _ => {
                                panic!("Expected left to be an int, right to be an int")
                            }
                        }
                    }
                    _ => {
                        panic!("Expected left to be an int, right to be a binary op")
                    }
                }
            }
            _ => panic!("Expected a binary op, got {:?}", result),
        }
    }
}
