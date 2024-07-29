use super::{
    token::{self, Token, TokenType},
    utils, LexResult,
};

mod identifier;
mod new_comment;
mod new_line;
mod number;
mod operator;
mod string;

// This module contains the individual scanners, and exports them

/// Attempts to scan a number. If not found returns None to be able to chain other scanner
pub fn number(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    utils::is_digit(c).then(|| number::scan(chars, start_pos))
}

/// Attempts to scan an operator. If not found returns None to be able to chain other scanner
pub fn operator(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    utils::is_operator(c).then(|| operator::scan(chars, start_pos))
}

/// Attempts to scan a grouping sign. If not found returns None to be able to chain other scanner
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

    let token = Token::new(c.to_string(), start_pos, token_type);
    Some(LexResult::Some(token, start_pos + 1))
}

/// Attempts to scan an identifier. If not found returns None to be able to chain other scanner
pub fn identifier(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    (utils::is_lowercase(c) || c == '_').then(|| identifier::scan(c, chars, start_pos))
}

/// Attempts to scan a datatype. If not found returns None to be able to chain other scanner
pub fn datatype(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    // Since the only difference with an identifier is that the fist character is an
    // uppercase letter, reuse the identifier scanner
    utils::is_uppercase(c).then(|| identifier::scan(c, chars, start_pos))
}

/// Attempts to scan a string. If not found returns None to be able to chain other scanner
pub fn string(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    (c == '"').then(|| string::scan(chars, start_pos + 1))
}

/// Attemts to scan a new line. If not found returns None to be able to chain other scanner
pub fn new_line(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    (c == '\n').then(|| new_line::scan(chars, start_pos))
}

/// Attempts to scan a single line comment.
pub fn new_comment(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    let next_char = chars.get(start_pos + 1);
    match (c, next_char) {
        ('/', Some('/')) => Some(new_comment::scan(chars, start_pos)),
        _ => None,
    }
}

pub fn new_multiline_comment(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    let next_char = chars.get(start_pos + 1);
    match (c, next_char) {
        ('/', Some('*')) => Some(new_comment::scan_multiline(chars, start_pos)),
        _ => None,
    }
}
