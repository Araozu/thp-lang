#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Identifier,
    Datatype,
    Number,
    String,
    Operator,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    NewLine,
    Comment,
    Comma,
    INDENT,
    DEDENT,
    VAL,
    VAR,
    EOF,
    FUN,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    // The token as a raw string
    pub value: String,
    /// The absolute position of this token, from the
    /// start of the file
    pub position: usize,
}

impl Token {
    pub fn get_end_position(&self) -> usize {
        self.position + self.value.len()
    }
}

impl Token {
    pub fn new_eof(position: usize) -> Token {
        Token {
            token_type: TokenType::EOF,
            value: String::from(""),
            position,
        }
    }

    pub fn new_number(value: String, position: usize) -> Token {
        Token {
            token_type: TokenType::Number,
            value,
            position,
        }
    }

    pub fn new_operator(value: String, position: usize) -> Token {
        Token {
            token_type: TokenType::Operator,
            value,
            position,
        }
    }

    pub fn new(value: String, position: usize, token_type: TokenType) -> Token {
        Token {
            token_type,
            value,
            position,
        }
    }

    pub fn new_identifier(value: String, position: usize) -> Token {
        Token {
            token_type: TokenType::Identifier,
            value,
            position,
        }
    }

    pub fn new_string(value: String, position: usize) -> Token {
        Token {
            token_type: TokenType::String,
            value,
            position,
        }
    }

    pub fn new_datatype(value: String, position: usize) -> Token {
        Token {
            token_type: TokenType::Datatype,
            value,
            position,
        }
    }

    pub fn new_comment(value: String, position: usize) -> Token {
        Token {
            token_type: TokenType::Comment,
            value,
            position,
        }
    }

    pub fn new_indent(position: usize) -> Token {
        Token {
            token_type: TokenType::INDENT,
            value: String::from(""),
            position,
        }
    }

    pub fn new_dedent(position: usize) -> Token {
        Token {
            token_type: TokenType::DEDENT,
            value: String::from(""),
            position,
        }
    }
}
