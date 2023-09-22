use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
    syntax::utils::parse_token_type,
};

use super::{ast::ParamsList, ParseResult};

pub fn parse_params_list<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
) -> ParseResult<ParamsList, &Token> {
    let mut current_pos = pos;

    let (opening_paren, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::LeftParen) {
            ParseResult::Ok(t, next) => (t, next),
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Mismatch(t) => return ParseResult::Mismatch(t),
            ParseResult::Unmatched => return ParseResult::Unmatched,
        };
    current_pos = next_pos;

    // Parse closing paren
    let (_closing_paren, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::RightParen) {
            ParseResult::Ok(t, next) => (t, next),
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Mismatch(t) => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from("Expected a closing paren after the function identifier."),
                    error_start: t.position,
                    error_end: t.get_end_position(),
                });
            }
            ParseResult::Unmatched => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from("Expected a closing paren after the function identifier."),
                    error_start: opening_paren.position,
                    error_end: opening_paren.get_end_position(),
                });
            }
        };
    current_pos = next_pos;

    ParseResult::Ok(ParamsList {}, current_pos)
}
