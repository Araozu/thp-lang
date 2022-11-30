mod utils;
mod scanner;
use super::token::{self, Token};

type Chars = Vec<char>;

pub enum LexResult {
    // A token was scanned
    Some(Token, usize),
    // No token was found, but there was no error (EOF)
    None(usize),
    Err(String),
}


/// Scans and returns all the tokens in the input String
pub fn get_tokens(input: &String) -> Result<Vec<Token>, String> {
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

    results.push(token::new_eof(0));
    Ok(results)
}

fn next_token(chars: &Chars, current_pos: usize) -> LexResult {
    let next_char = peek(chars, current_pos);

    // If EOF is reached return nothing but the current position
    if next_char == '\0' {
        return LexResult::None(current_pos)
    }

    // Handle whitespace recursively
    if next_char == ' ' {
        return next_token(chars, current_pos + 1)
    }

    // Test number
    None
        .or_else(|| {
            scanner::number(next_char, chars, current_pos)
        })
        .or_else(|| {
            scanner::operator(next_char, chars, current_pos)
        })
        .or_else(|| {
            scanner::grouping_sign(next_char, chars, current_pos)
        })
        .unwrap_or_else(|| {
            LexResult::Err(format!("Unrecognized character: {}", next_char))
        })
}

fn peek(input: &Chars, pos: usize) -> char {
    let result = input.get(pos).unwrap_or(&'\0');
    *result
}

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
        assert_eq!(1, tokens.len());
        let first = tokens.get(0).unwrap();
        assert_eq!(TokenType::EOF, first.token_type);

        let input = String::from("  ");
        let tokens = get_tokens(&input).unwrap();
        assert_eq!(1, tokens.len());
        let first = tokens.get(0).unwrap();
        assert_eq!(TokenType::EOF, first.token_type);

        let input = String::from("    ");
        let tokens = get_tokens(&input).unwrap();
        assert_eq!(1, tokens.len());
        let first = tokens.get(0).unwrap();
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
        assert_eq!(TokenType::EOF, tokens.get(5).unwrap().token_type);
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
}
