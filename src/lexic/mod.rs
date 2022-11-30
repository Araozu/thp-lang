mod utils;
mod scanner;
use super::token::{self, Token};

type Chars = Vec<char>;

/// Scans and returns all the tokens in the input String
pub fn get_tokens(input: &String) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().into_iter().collect();
    let mut results = Vec::new();
    let mut current_pos: usize = 0;

    while has_input(&chars, current_pos) {
        match next_token(&chars, current_pos) {
            Ok((Some(token), next_pos)) => {
                results.push(token);
                current_pos = next_pos;
            },
            Ok((None, next_pos)) => {
                current_pos = next_pos;
            },
            Err(reason) => return Err(reason),
        }
    }

    results.push(token::new_eof(0));
    Ok(results)
}

fn next_token(chars: &Chars, current_pos: usize) -> Result<(Option<Token>, usize),String> {
    let next_char = peek(chars, current_pos);

    // If EOF is reached return nothing
    if next_char == '\0' {
        return Ok((None, current_pos))
    }

    // Handle whitespace recursively
    if next_char == ' ' {
        return next_token(chars, current_pos + 1)
    }

    // Test number
    if utils::is_digit(next_char) {
        match scanner::number(chars, current_pos) {
            Ok((token, next_pos)) => Ok((Some(token), next_pos)),
            Err(reason) => Err(reason),
        }
    }
    // Test operator
    else if utils::is_operator(next_char) {
        let (token, next_pos) = scanner::operator(chars, current_pos);
        Ok((Some(token), next_pos))
    }
    else {
        Err(format!("Unrecognized character: {}", next_char))
    }
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

        match next_token(&chars, 0).unwrap() {
            (Some(t), _) => {
                assert_eq!("126", t.value)
            },
            (None, _) => {
                panic!()
            }
        }
    }

    /// Should scan numbers
    #[test]
    fn number_test() {
        let input = String::from("126 278.98 0.282398");
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
        /*
        assert_eq!("1798e+1", tokens.get(3).unwrap().value);
        assert_eq!("239.3298e-103", tokens.get(4).unwrap().value);
        assert_eq!(TokenType::EOF, tokens.get(5).unwrap().token_type);
        */
    }
}
