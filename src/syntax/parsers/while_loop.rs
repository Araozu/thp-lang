use crate::{
    error_handling::SyntaxError,
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
                return Err(ParsingError::Err(SyntaxError {
                    error_start: e.position,
                    error_end: e.get_end_position(),
                    reason: format!(
                        "Expected an expression after the `while` keyword, found {}",
                        e.value
                    ),
                }))
            }
            Err(ParsingError::Unmatched) => {
                return Err(ParsingError::Err(SyntaxError {
                    error_start: while_keyword.position,
                    error_end: while_keyword.get_end_position(),
                    reason: format!("Expected an identifier after the `while` keyword"),
                }))
            }
        };

        // body
        let (body, next) = match Block::try_parse(tokens, next) {
            Ok(t) => t,
            Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
            Err(ParsingError::Mismatch(e)) => {
                return Err(ParsingError::Err(SyntaxError {
                    error_start: e.position,
                    error_end: e.get_end_position(),
                    reason: format!("Expected a block after the condition, found {}", e.value),
                }))
            }
            Err(ParsingError::Unmatched) => {
                return Err(ParsingError::Err(SyntaxError {
                    error_start: while_keyword.position,
                    error_end: while_keyword.get_end_position(),
                    reason: format!("Expected a block after the condition"),
                }))
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
