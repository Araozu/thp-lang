mod utils;
mod scanner;
use super::token::{self, Token};

type Chars = Vec<char>;

/// Scans and returns all the tokens in the input String
pub fn get_tokens(input: &String) -> Vec<Token> {
    let chars: Vec<char> = input.chars().into_iter().collect();
    let mut results = Vec::new();
    let mut current_pos: usize = 0;

    while has_input(&chars, current_pos) {
        let (possible_token, next_pos) = next_token(&chars, current_pos);
        current_pos = next_pos;

        if let Some(token) = possible_token {
            results.push(token);
        }
    }

    results.push(token::new_eof(0));
    results
}

fn next_token(chars: &Chars, current_pos: usize) -> (Option<Token>, usize) {
    let next_char = peek(chars, current_pos);

    // Handle whitespace
    if next_char == ' ' {
        return next_token(chars, current_pos + 1)
    }

    // Test number
    if utils::is_digit(next_char) {
        let (token, next_pos) = scanner::number(chars, current_pos).unwrap();
        (Some(token), next_pos)
    }
    // Test operator
    else if utils::is_operator(next_char) {
        let (token, next_pos) = scanner::operator(chars, current_pos);
        (Some(token), next_pos)
    }
    else {
        (None, current_pos)
    }
}

fn peek(input: &Chars, pos: usize) -> char {
    let result = input.get(pos).unwrap_or(&'\0');
    *result
}

fn has_input(input: &Chars, current_pos: usize) -> bool {
    input.len() < current_pos
}



#[cfg(test)]
mod tests {
    use super::*;
    use token::{Token, TokenType};

    /// Should return an EOF token if the input has no tokens
    #[test]
    fn test1() {
        let input = String::from("");
        let tokens = get_tokens(&input);
        assert_eq!(1, tokens.len());
        let first = tokens.get(0).unwrap();
        assert_eq!(TokenType::EOF, first.token_type);

        let input = String::from("  ");
        let tokens = get_tokens(&input);
        assert_eq!(1, tokens.len());
        let first = tokens.get(0).unwrap();
        assert_eq!(TokenType::EOF, first.token_type);

        let input = String::from("  \n  ");
        let tokens = get_tokens(&input);
        assert_eq!(1, tokens.len());
        let first = tokens.get(0).unwrap();
        assert_eq!(TokenType::EOF, first.token_type);
    }

    /// Should scan numbers
    #[test]
    fn number_test() {
        let input = String::from("126 278.98 0.282398 1798e+1 239.3298e-103");
        let tokens = get_tokens(&input);

        // assert_eq!("126", tokens.get(0).unwrap().value);
        /*
        assert_eq!("278.98", tokens.get(1).unwrap().value);
        assert_eq!("0.282398", tokens.get(2).unwrap().value);
        assert_eq!("1798e+1", tokens.get(3).unwrap().value);
        assert_eq!("239.3298e-103", tokens.get(4).unwrap().value);
        assert_eq!(TokenType::EOF, tokens.get(5).unwrap().token_type);
        */
    }
}
