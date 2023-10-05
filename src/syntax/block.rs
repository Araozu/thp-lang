use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
};

use super::{ast::Block, utils::parse_token_type, ParseResult};

// Assumes that the token at `pos` is a {
pub fn parse_block<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParseResult<Block, &Token> {
    let mut current_pos = pos;

    let (opening_brace, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::LeftBrace) {
            ParseResult::Ok(t, next) => (t, next),
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Mismatch(t) => return ParseResult::Mismatch(t),
            ParseResult::Unmatched => return ParseResult::Unmatched,
        };
    current_pos = next_pos;

    // Parse block statements
    let mut statements = Vec::new();

    // Only 1 statement for now
    match super::statement::try_parse(tokens, current_pos) {
        ParseResult::Ok(statement, next_pos) => {
            current_pos = next_pos;
            statements.push(statement);
        }
        ParseResult::Err(err) => return ParseResult::Err(err),
        ParseResult::Unmatched => {}
        ParseResult::Mismatch(_) => {}
    }

    // Parse closing brace
    let (_closing_brace, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::RightBrace) {
            ParseResult::Ok(t, next) => (t, next),
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Mismatch(t) => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from("Expected a closing brace after the block body."),
                    error_start: t.position,
                    error_end: t.get_end_position(),
                });
            }
            ParseResult::Unmatched => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from("Expected a closing brace after the block body."),
                    error_start: opening_brace.position,
                    error_end: opening_brace.get_end_position(),
                });
            }
        };
    current_pos = next_pos;

    ParseResult::Ok(Block { statements }, current_pos)
}
