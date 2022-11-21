#[derive(PartialEq, Debug)]
pub enum TokenType {
    NewLine,
    Identifier,
    Comment,
    Number,
    String,
    Unit,
    Operator,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Indent,
    Dedent,
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
    position: i32,
}

pub fn new_eof(position: i32) -> Token {
    Token {
        token_type: TokenType::EOF,
        value: String::from(""),
        position,
    }
}

pub fn new_number(value: String, position: i32) -> Token {
    Token {
        token_type: TokenType::Number,
        value,
        position
    }
}
