use crate::{
    error_handling::{error_messages::SYNTAX_INVALID_WHILE_LOOP, ErrorContainer, ErrorLabel},
    lexic::token::{Token, TokenType},
    syntax::{
        ast::{loops::WhileLoop, Block, Expression, Positionable},
        parseable::{Parseable, ParsingError, ParsingResult},
        utils::parse_token_type,
    },
};

impl<'a> Parseable<'a> for WhileLoop<'a> {
    type Item = WhileLoop<'a>;

    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        // while keyword
        let (while_keyword, next) = match parse_token_type(tokens, current_pos, TokenType::WHILE) {
            Ok(tuple) => tuple,
            _ => return Err(ParsingError::Unmatched),
        };

        // condition expression
        let (condition, next) = match Expression::try_parse(tokens, next) {
            Ok(t) => t,
            Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
            Err(ParsingError::Mismatch(e)) => {
                let label = ErrorLabel {
                    message: String::from("Expected a Bool expression here"),
                    start: e.position,
                    end: e.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_WHILE_LOOP,
                    error_offset: e.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            Err(ParsingError::Unmatched) => {
                let label = ErrorLabel {
                    message: String::from("Expected a Bool expression after this `while` keyword"),
                    start: while_keyword.position,
                    end: while_keyword.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_WHILE_LOOP,
                    error_offset: while_keyword.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };

        // body
        let (body, next) = match Block::try_parse(tokens, next) {
            Ok(t) => t,
            Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
            Err(ParsingError::Mismatch(e)) => {
                let label = ErrorLabel {
                    message: String::from("Expected a block here, after the condition"),
                    start: e.position,
                    end: e.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_WHILE_LOOP,
                    error_offset: e.position,
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
                    error_code: SYNTAX_INVALID_WHILE_LOOP,
                    error_offset: error_start,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };

        // return

        let (_, loop_end) = body.get_position();
        let while_loop = WhileLoop {
            loop_start: while_keyword.position,
            loop_end,
            condition,
            body,
        };
        Ok((while_loop, next))
    }
}
