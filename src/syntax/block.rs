use crate::{
    lexic::token::{Token, TokenType},
    syntax::{utils::expect_token_w, SyntaxResult},
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

    // Parse closing brace
    let (_closing_brace, next_pos) = match expect_token_w(
        tokens,
        current_pos,
        TokenType::RightBrace,
        "Expected a closing brace after the block body.".into(),
        opening_brace,
    ) {
        Ok(t) => t,
        Err(Some(SyntaxResult::Err(err))) => return ParseResult::Err(err),
        _ => panic!("parse_block: invalid state"),
    };
    current_pos = next_pos;

    ParseResult::Ok(Block {}, current_pos)
}
