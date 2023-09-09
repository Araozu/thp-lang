use crate::{
    lexic::token::{Token, TokenType},
    utils::Result3,
};

pub fn try_token_type(tokens: &Vec<Token>, pos: usize, token_type: TokenType) -> Result3<&Token> {
    match tokens.get(pos) {
        Some(t) if t.token_type == token_type => Result3::Ok(t),
        Some(t) if t.token_type == TokenType::Semicolon || t.token_type == TokenType::EOF => {
            Result3::None
        }
        Some(t) => Result3::Err(t),
        None => Result3::None,
    }
}
