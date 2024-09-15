use crate::{
    error_handling::{error_messages::SYNTAX_INVALID_FOR_LOOP, ErrorContainer, ErrorLabel},
    lexic::token::{Token, TokenType},
    syntax::{
        ast::{loops::ForLoop, Block, Expression, Positionable},
        parseable::{Parseable, ParsingError, ParsingResult},
        utils::parse_token_type,
    },
};

impl<'a> Parseable<'a> for ForLoop<'a> {
    type Item = ForLoop<'a>;

    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        // for keyword
        let (for_keyword, next) = match parse_token_type(tokens, current_pos, TokenType::FOR) {
            Ok(tuple) => tuple,
            _ => return Err(ParsingError::Unmatched),
        };

        // first identifier
        let (first_id, next) = match parse_token_type(tokens, next, TokenType::Identifier) {
            Ok(t) => t,
            Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
            Err(ParsingError::Mismatch(e)) => {
                let label = ErrorLabel {
                    message: String::from("Expected an identifier here, after the `for` keyword"),
                    start: e.position,
                    end: e.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_FOR_LOOP,
                    error_offset: e.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            Err(ParsingError::Unmatched) => {
                let label = ErrorLabel {
                    message: String::from("Expected an identifier after this `for` keyword"),
                    start: for_keyword.position,
                    end: for_keyword.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_FOR_LOOP,
                    error_offset: for_keyword.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };

        // comma and possible second identifier
        let (second_id, next) = 'block: {
            // attempt to parse comma
            let (comma, next) = match parse_token_type(tokens, next, TokenType::Comma) {
                Ok(t) => t,
                _ => break 'block (None, next),
            };

            // parse second id
            // if this fails then its a syntax error, because a comma was already commited
            match parse_token_type(&tokens, next, TokenType::Identifier) {
                Ok((second_id, next)) => (Some(second_id), next),
                Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
                Err(ParsingError::Mismatch(t)) => {
                    let label = ErrorLabel {
                        message: String::from("Expected an identifier here, after the comma"),
                        start: t.position,
                        end: t.get_end_position(),
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INVALID_FOR_LOOP,
                        error_offset: t.position,
                        labels: vec![label],
                        note: Some(String::from(
                            "To iterate only over values, use `for identifier in ...`",
                        )),
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
                }
                Err(ParsingError::Unmatched) => {
                    let label = ErrorLabel {
                        message: String::from("Expected an identifier after this comma"),
                        start: comma.position,
                        end: comma.get_end_position(),
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INVALID_FOR_LOOP,
                        error_offset: comma.position,
                        labels: vec![label],
                        note: Some(String::from(
                            "To iterate only over values, use `for identifier in ...`",
                        )),
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
                }
            }
        };

        // in keyword
        let (in_keyword, next) = match parse_token_type(tokens, next, TokenType::IN) {
            Ok(tuple) => tuple,
            Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
            Err(ParsingError::Mismatch(t)) => {
                let label = ErrorLabel {
                    message: String::from("Expected the `in` keyword here"),
                    start: t.position,
                    end: t.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_FOR_LOOP,
                    error_offset: t.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            Err(ParsingError::Unmatched) => {
                let previous_token = if second_id.is_none() {
                    first_id
                } else {
                    second_id.unwrap()
                };
                let label = ErrorLabel {
                    message: String::from("Expected the `in` keyword after this identifier"),
                    start: previous_token.position,
                    end: previous_token.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_FOR_LOOP,
                    error_offset: previous_token.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };

        // expression
        let (expr, next) = match Expression::try_parse(tokens, next) {
            Ok(t) => t,
            Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
            Err(_) => {
                let label = ErrorLabel {
                    message: String::from("Expected an expression after this `in` keyword"),
                    start: in_keyword.position,
                    end: in_keyword.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_FOR_LOOP,
                    error_offset: in_keyword.position,
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
                    message: String::from("Expected a block here"),
                    start: wrong_token.position,
                    end: wrong_token.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_FOR_LOOP,
                    error_offset: wrong_token.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            Err(ParsingError::Unmatched) => {
                let (error_start, error_end) = expr.get_position();
                let label = ErrorLabel {
                    message: String::from("Expected a block here, after the collection"),
                    start: error_start,
                    end: error_end,
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_FOR_LOOP,
                    error_offset: error_start,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };

        // return
        let (key, value) = match second_id {
            Some(id) => (Some(first_id), id),
            None => (None, first_id),
        };

        let (_, loop_end) = block.get_position();
        let for_loop = ForLoop {
            loop_start: for_keyword.position,
            loop_end,
            key,
            value,
            collection: expr,
            body: block,
        };
        Ok((for_loop, next))
    }
}
