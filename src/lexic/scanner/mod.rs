use super::{token::{TokenType, self}, utils, LexResult};

mod number;
mod operator;
mod identifier;

/// Attempts to scan a number. Returns None to be able to chain other scanner
pub fn number(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    utils::is_digit(c).then(|| number::scan(chars, start_pos))
}


/// Attempts to scan an operator. Returns None to be able to chain other scanner
pub fn operator(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    utils::is_operator(c).then(|| operator::scan(chars, start_pos))
}


/// Attempts to scan a grouping sign. Returns None to be able to chain other scanner
pub fn grouping_sign(c: char, _: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    let token_type = match c {
        '(' => TokenType::LeftParen,
        ')' => TokenType::RightParen,
        '[' => TokenType::LeftBracket,
        ']' => TokenType::RightBracket,
        '{' => TokenType::LeftBrace,
        '}' => TokenType::RightBrace,
        _ => return None,
    };

    let token = token::new(
        c.to_string(), 
        start_pos as i32, 
        token_type,
    );
    Some(LexResult::Some(token, start_pos + 1))
}


/// Attempts to scan an identifier. Returns None to be able to chain other scanner
pub fn identifier(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    utils::is_lowercase(c).then(|| identifier::scan(c, chars, start_pos))
}
