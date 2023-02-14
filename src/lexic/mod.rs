mod utils;
mod scanner;
mod lex_error;
use super::token::{self, Token};
use lex_error::LexError;

type Chars = Vec<char>;

/// Represents the result of scanning a single token from the input
pub enum LexResult {
    /// A token was found. The first element is the token, and the
    /// second element is the position in the input after the token.
    ///
    /// E.g., given an input 
    ///
    /// "`identifier 55`"
    ///
    /// scanning from a position `0`, the result would be
    ///
    /// `Some(Token("identifier"), 10)`.
    ///
    /// where:
    /// - `Token("identifier")` is the token
    /// - `10` is the position where the token ends, and from where the next token
    /// should be scanned
    Some(Token, usize),
    /// No token was found. This indicates that EOF has been reached.
    ///
    /// Contains the last position, which should be the input lenght - 1
    None(usize),
    /// An error was found while scanning.
    Err(LexError),
}


/// Scans and returns all the tokens in the input String
pub fn get_tokens(input: &String) -> Result<Vec<Token>, LexError> {
    let chars: Vec<char> = input.chars().into_iter().collect();
    let mut results = Vec::new();
    let mut current_pos: usize = 0;

    while has_input(&chars, current_pos) {
        match next_token(&chars, current_pos) {
            LexResult::Some(token, next_pos) => {
                results.push(token);
                current_pos = next_pos;
            },
            LexResult::None(next_pos) => {
                current_pos = next_pos;
            },
            LexResult::Err(reason) => return Err(reason),
        }
    }

    results.push(token::new_semicolon(0));
    results.push(token::new_eof(0));
    Ok(results)
}

/// Scans a single token from `chars`, starting from `current_pos`
fn next_token(chars: &Chars, current_pos: usize) -> LexResult {
    let next_char = peek(chars, current_pos);

    // If EOF is reached return nothing but the current position
    if next_char == '\0' {
        return LexResult::None(current_pos)
    }

    // Handle whitespace recursively.
    if next_char == ' ' {
        return next_token(chars, current_pos + 1)
    }

    // Scanners
    None
        .or_else(|| scanner::number(next_char, chars, current_pos))
        .or_else(|| scanner::identifier(next_char, chars, current_pos))
        .or_else(|| scanner::string(next_char, chars, current_pos))
        .or_else(|| scanner::operator(next_char, chars, current_pos))
        .or_else(|| scanner::grouping_sign(next_char, chars, current_pos))
        .or_else(|| scanner::new_line(next_char, chars, current_pos))
        .unwrap_or_else(|| {
            let error = LexError {
                position: current_pos,
                reason: format!("Unrecognized character: {}", next_char),
            };
            LexResult::Err(error)
        })
}

/// Returns the char at `pos`
fn peek(input: &Chars, pos: usize) -> char {
    let result = input.get(pos).unwrap_or(&'\0');
    *result
}

/// Whether there is still input based on `current_pos`
fn has_input(input: &Chars, current_pos: usize) -> bool {
    current_pos < input.len()
}



#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;

    /// Should return an EOF token if the input has no tokens
    #[test]
    fn test1() {
        let input = String::from("");
        let tokens = get_tokens(&input).unwrap();
        // 1 semicolon and 1 EOF token
        assert_eq!(2, tokens.len());
        let first = tokens.get(1).unwrap();
        assert_eq!(TokenType::EOF, first.token_type);

        let input = String::from("  ");
        let tokens = get_tokens(&input).unwrap();
        // 1 semicolon and 1 EOF token
        assert_eq!(2, tokens.len());
        let first = tokens.get(1).unwrap();
        assert_eq!(TokenType::EOF, first.token_type);

        let input = String::from("    ");
        let tokens = get_tokens(&input).unwrap();
        // 1 semicolon and 1 EOF token
        assert_eq!(2, tokens.len());
        let first = tokens.get(1).unwrap();
        assert_eq!(TokenType::EOF, first.token_type);
    }

    #[test]
    fn t() {
        let input = String::from("126 ");
        let chars: Vec<char> = input.chars().into_iter().collect();

        assert_eq!(4, chars.len());
        assert!(has_input(&chars, 0));

        match next_token(&chars, 0) {
            LexResult::Some(t, _) => {
                assert_eq!("126", t.value)
            },
            _ => {
                panic!()
            }
        }
    }

    /// Should scan numbers
    #[test]
    fn number_test() {
        let input = String::from("126 278.98 0.282398 1789e+1 239.3298e-103");
        let tokens = get_tokens(&input).unwrap();

        let t1 = tokens.get(0).unwrap();
        assert_eq!(TokenType::Number, t1.token_type);
        assert_eq!("126", t1.value);

        let t2 = tokens.get(1).unwrap();
        assert_eq!(TokenType::Number, t2.token_type);
        assert_eq!("278.98", t2.value);

        let t3 = tokens.get(2).unwrap();
        assert_eq!(TokenType::Number, t3.token_type);
        assert_eq!("0.282398", t3.value);
        
        assert_eq!("1789e+1", tokens.get(3).unwrap().value);
        assert_eq!("239.3298e-103", tokens.get(4).unwrap().value);
        assert_eq!(TokenType::Semicolon, tokens.get(5).unwrap().token_type);
        assert_eq!(TokenType::EOF, tokens.get(6).unwrap().token_type);
    }

    #[test]
    fn grouping_sign_test() {
        let input = String::from("( ) { } [ ]");
        let tokens = get_tokens(&input).unwrap();

        let t = tokens.get(0).unwrap();
        assert_eq!(TokenType::LeftParen, t.token_type);
        assert_eq!("(", t.value);

        let t = tokens.get(1).unwrap();
        assert_eq!(TokenType::RightParen, t.token_type);
        assert_eq!(")", t.value);

        let t = tokens.get(2).unwrap();
        assert_eq!(TokenType::LeftBrace, t.token_type);
        assert_eq!("{", t.value);

        let t = tokens.get(3).unwrap();
        assert_eq!(TokenType::RightBrace, t.token_type);
        assert_eq!("}", t.value);

        let t = tokens.get(4).unwrap();
        assert_eq!(TokenType::LeftBracket, t.token_type);
        assert_eq!("[", t.value);

        let t = tokens.get(5).unwrap();
        assert_eq!(TokenType::RightBracket, t.token_type);
        assert_eq!("]", t.value);
    }
    
    #[test]
    fn should_scan_new_line() {
        let input = String::from("3\n22");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Semicolon, tokens[1].token_type);
    }
    
    #[test]
    fn should_scan_multiple_new_lines() {
        let input = String::from("3\n\n\n22");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Semicolon, tokens[1].token_type);
        assert_eq!(TokenType::Number, tokens[2].token_type);
    }
    
    #[test]
    fn should_scan_multiple_new_lines_with_whitespace_in_between() {
        let input = String::from("3\n \n   \n22");
        let tokens = get_tokens(&input).unwrap();

        assert_eq!(TokenType::Semicolon, tokens[1].token_type);
        assert_eq!(TokenType::Number, tokens[2].token_type);
    }
}
