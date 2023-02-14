#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Identifier,
    Number,
    String,
    Operator,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    VAR,
    VAL,
    EOF,
}

pub struct Token {
    pub token_type: TokenType,
    // The token as a raw string
    pub value: String,
    /// The absolute position of this token, from the
    /// start of the file
    _position: i32,
}

pub fn new_eof(position: i32) -> Token {
    Token {
        token_type: TokenType::EOF,
        value: String::from(""),
        _position: position,
    }
}

pub fn new_number(value: String, position: i32) -> Token {
    Token {
        token_type: TokenType::Number,
        value,
        _position: position
    }
}

pub fn new_operator(value: String, position: i32) -> Token {
    Token {
        token_type: TokenType::Operator,
        value,
        _position: position
    }
}

pub fn new(value: String, position: i32, token_type: TokenType) -> Token {
    Token {token_type, value, _position: position}
}

pub fn new_identifier(value: String, position: i32) -> Token {
    Token {
        token_type: TokenType::Identifier,
        value,
        _position: position,
    }
}

pub fn new_string(value: String, position: i32) -> Token {
    Token {
        token_type: TokenType::String,
        value,
        _position: position,
    }
}
