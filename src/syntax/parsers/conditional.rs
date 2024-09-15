use crate::{
    error_handling::{error_messages::SYNTAX_INVALID_IF_CONDITION, ErrorContainer, ErrorLabel},
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
                let label = ErrorLabel {
                    message: String::from("Expected a Bool expression here"),
                    start: wrong_token.position,
                    end: wrong_token.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_IF_CONDITION,
                    error_offset: wrong_token.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            Err(ParsingError::Unmatched) => {
                let label = ErrorLabel {
                    message: String::from("Expected a Bool expression after this `if` keyword"),
                    start: if_token.position,
                    end: if_token.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_IF_CONDITION,
                    error_offset: if_token.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };

        // if block
        let (if_block, next) = match Block::try_parse(tokens, next) {
            Ok(t) => t,
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(wrong_token)) => {
                let label = ErrorLabel {
                    message: String::from("Expected a block here, after the condition"),
                    start: wrong_token.position,
                    end: wrong_token.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_IF_CONDITION,
                    error_offset: wrong_token.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            Err(ParsingError::Unmatched) => {
                let (error_start, error_end) = if_expression.get_position();
                let label = ErrorLabel {
                    message: String::from("Expected a block after this condition"),
                    start: error_start,
                    end: error_end,
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_IF_CONDITION,
                    error_offset: error_start,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
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
                    let label = ErrorLabel {
                        message: String::from("Expected a Bool expression here"),
                        start: wrong_token.position,
                        end: wrong_token.get_end_position(),
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INVALID_IF_CONDITION,
                        error_offset: wrong_token.position,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
                }
                Err(ParsingError::Unmatched) => {
                    let label = ErrorLabel {
                        message: String::from("Expected a Bool expression after this `if` keyword"),
                        start: if_token.position,
                        end: if_token.get_end_position(),
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INVALID_IF_CONDITION,
                        error_offset: if_token.position,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
                }
            };

            // block
            let (block, next) = match Block::try_parse(tokens, next) {
                Ok(t) => t,
                Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
                Err(ParsingError::Mismatch(wrong_token)) => {
                    let label = ErrorLabel {
                        message: String::from("Expected a block here, after the condition"),
                        start: wrong_token.position,
                        end: wrong_token.get_end_position(),
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INVALID_IF_CONDITION,
                        error_offset: wrong_token.position,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
                }
                Err(ParsingError::Unmatched) => {
                    let (error_start, error_end) = condition.get_position();
                    let label = ErrorLabel {
                        message: String::from("Expected a block after this condition"),
                        start: error_start,
                        end: error_end,
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INVALID_IF_CONDITION,
                        error_offset: error_start,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
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
                            let label = ErrorLabel {
                                message: String::from("Expected a block here, after the condition"),
                                start: wrong_token.position,
                                end: wrong_token.get_end_position(),
                            };
                            let econtainer = ErrorContainer {
                                error_code: SYNTAX_INVALID_IF_CONDITION,
                                error_offset: wrong_token.position,
                                labels: vec![label],
                                note: None,
                                help: None,
                            };
                            return Err(ParsingError::Err(econtainer));
                        }
                        Err(ParsingError::Unmatched) => {
                            let label = ErrorLabel {
                                message: String::from(
                                    "Expected a block here, after this `else` keyword",
                                ),
                                start: else_token.position,
                                end: else_token.get_end_position(),
                            };
                            let econtainer = ErrorContainer {
                                error_code: SYNTAX_INVALID_IF_CONDITION,
                                error_offset: else_token.position,
                                labels: vec![label],
                                note: None,
                                help: None,
                            };
                            return Err(ParsingError::Err(econtainer));
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
