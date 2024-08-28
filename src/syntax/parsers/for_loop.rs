use crate::{
    error_handling::SyntaxError,
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
                return Err(ParsingError::Err(SyntaxError {
                    error_start: e.position,
                    error_end: e.get_end_position(),
                    reason: format!(
                        "Expected an identifier after the `for` keyword, found {}",
                        e.value
                    ),
                }))
            }
            Err(ParsingError::Unmatched) => {
                return Err(ParsingError::Err(SyntaxError {
                    error_start: for_keyword.position,
                    error_end: for_keyword.get_end_position(),
                    reason: format!("Expected an identifier after the `for` keyword"),
                }))
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
                    return Err(ParsingError::Err(SyntaxError {
                        error_start: t.position,
                        error_end: t.get_end_position(),
                        reason: format!(
                            "Expected an identifier after the comma, found `{}`",
                            t.value
                        ),
                    }))
                }
                Err(ParsingError::Unmatched) => {
                    return Err(ParsingError::Err(SyntaxError {
                        error_start: comma.position,
                        error_end: comma.get_end_position(),
                        reason: format!("Expected an identifier after the comma"),
                    }));
                }
            }
        };

        // in keyword
        let (in_keyword, next) = match parse_token_type(tokens, next, TokenType::IN) {
            Ok(tuple) => tuple,
            Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
            Err(ParsingError::Mismatch(t)) => {
                return Err(ParsingError::Err(SyntaxError {
                    error_start: t.position,
                    error_end: t.get_end_position(),
                    reason: format!("Expected the `in` keyword, found `{}`", t.value),
                }))
            }
            Err(ParsingError::Unmatched) => {
                let previous_token = if second_id.is_none() {
                    first_id
                } else {
                    second_id.unwrap()
                };
                return Err(ParsingError::Err(SyntaxError {
                    error_start: previous_token.position,
                    error_end: previous_token.get_end_position(),
                    reason: format!("Expected the `in` keyword"),
                }));
            }
        };

        // expression
        let (expr, next) = match Expression::try_parse(tokens, next) {
            Ok(t) => t,
            Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
            Err(_) => {
                return Err(ParsingError::Err(SyntaxError {
                    error_start: in_keyword.position,
                    error_end: in_keyword.get_end_position(),
                    reason: format!("Expected an expression after the `in` keyword"),
                }))
            }
        };

        // block
        let (block, next) = match Block::try_parse(tokens, next) {
            Ok(t) => t,
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(wrong_token)) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected a block after the collection"),
                    error_start: wrong_token.position,
                    error_end: wrong_token.get_end_position(),
                }));
            }
            Err(ParsingError::Unmatched) => {
                let (error_start, error_end) = expr.get_position();
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected a block after the collection"),
                    error_start,
                    error_end,
                }));
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
            body: block,
        };
        Ok((for_loop, next))
    }
}
