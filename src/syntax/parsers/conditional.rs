use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
    syntax::{
        ast::{Block, Condition, Conditional, Expression, Positionable},
        parseable::{Parseable, ParsingError, ParsingResult},
        utils::parse_token_type,
    },
};

impl<'a> Parseable<'a> for Conditional<'a> {
    type Item = Conditional<'a>;

    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        // if keyword
        let (if_token, next) = match parse_token_type(tokens, current_pos, TokenType::IF) {
            Ok(tuple) => tuple,
            _ => return Err(ParsingError::Unmatched),
        };

        // if condition
        let (if_expression, next) = match Expression::try_parse(tokens, next) {
            Ok(tuple) => tuple,
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(wrong_token)) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected an expression after the if token"),
                    error_start: wrong_token.position,
                    error_end: wrong_token.get_end_position(),
                }));
            }
            Err(ParsingError::Unmatched) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected an expression after the if token"),
                    error_start: if_token.position,
                    error_end: if_token.get_end_position(),
                }));
            }
        };

        // if block
        let (if_block, next) = match Block::try_parse(tokens, next) {
            Ok(t) => t,
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(wrong_token)) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected a block after the condition"),
                    error_start: wrong_token.position,
                    error_end: wrong_token.get_end_position(),
                }));
            }
            Err(ParsingError::Unmatched) => {
                let (error_start, error_end) = if_expression.get_position();
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected a block after the condition"),
                    error_start,
                    error_end,
                }));
            }
        };

        let mut else_if_members = Vec::<Condition>::new();
        let mut current_pos = next;
        let tokens_len = tokens.len();

        // many else if
        while current_pos < tokens_len {
            // else token
            let (_, next) = match parse_token_type(tokens, current_pos, TokenType::ELSE) {
                Ok(tuple) => tuple,
                _ => {
                    break;
                }
            };

            // if token
            let (if_token, next) = match parse_token_type(tokens, next, TokenType::IF) {
                Ok(tuple) => tuple,
                // This might be a else {}, not a else if {}
                _ => {
                    break;
                }
            };

            // condition
            let (condition, next) = match Expression::try_parse(tokens, next) {
                Ok(tuple) => tuple,
                Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
                Err(ParsingError::Mismatch(wrong_token)) => {
                    return Err(ParsingError::Err(SyntaxError {
                        reason: String::from("Expected an expression after the if token"),
                        error_start: wrong_token.position,
                        error_end: wrong_token.get_end_position(),
                    }));
                }
                Err(ParsingError::Unmatched) => {
                    return Err(ParsingError::Err(SyntaxError {
                        reason: String::from("Expected an expression after the if token"),
                        error_start: if_token.position,
                        error_end: if_token.get_end_position(),
                    }));
                }
            };

            // block
            let (block, next) = match Block::try_parse(tokens, next) {
                Ok(t) => t,
                Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
                Err(ParsingError::Mismatch(wrong_token)) => {
                    return Err(ParsingError::Err(SyntaxError {
                        reason: String::from("Expected a block after the condition"),
                        error_start: wrong_token.position,
                        error_end: wrong_token.get_end_position(),
                    }));
                }
                Err(ParsingError::Unmatched) => {
                    let (error_start, error_end) = condition.get_position();
                    return Err(ParsingError::Err(SyntaxError {
                        reason: String::from("Expected a block after the condition"),
                        error_start,
                        error_end,
                    }));
                }
            };

            else_if_members.push(Condition {
                condition,
                body: block,
            });

            current_pos = next;
        }

        // else
        let (else_block, next) = {
            match parse_token_type(tokens, current_pos, TokenType::ELSE) {
                Ok((else_token, next)) => {
                    // block
                    let (block, next) = match Block::try_parse(tokens, next) {
                        Ok(t) => t,
                        Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
                        Err(ParsingError::Mismatch(wrong_token)) => {
                            return Err(ParsingError::Err(SyntaxError {
                                reason: String::from("Expected a block after the else keyword"),
                                error_start: wrong_token.position,
                                error_end: wrong_token.get_end_position(),
                            }));
                        }
                        Err(ParsingError::Unmatched) => {
                            return Err(ParsingError::Err(SyntaxError {
                                reason: String::from("Expected a block after the else keyword"),
                                error_start: else_token.position,
                                error_end: else_token.get_end_position(),
                            }));
                        }
                    };

                    (Some(block), next)
                }
                _ => (None, current_pos),
            }
        };

        // return

        let result = Conditional {
            if_member: Condition {
                condition: if_expression,
                body: if_block,
            },
            else_if_members,
            else_block,
        };

        Ok((result, next))
    }
}
