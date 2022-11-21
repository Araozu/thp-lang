use crate::syntax::{token::{Token, self}, utils};

pub fn scan(chars: &Vec<char>, start_pos: usize) -> (Token, usize) {
    scan_decimal(chars, start_pos, String::from(""))
}

fn scan_decimal(chars: &Vec<char>, start_pos: usize, current: String) -> (Token, usize) {
    let next_char = chars.get(start_pos);

    if let Some(c) = next_char {
        if utils::is_digit(*c) {
            let new_value = format!("{}{}", current, *c);
            scan_decimal(chars, start_pos + 1, new_value)
        } else {
            (token::new_number(current, start_pos as i32), start_pos)
        }
    } else {
        (token::new_number(current, start_pos as i32), start_pos)
    }
}



#[cfg(test)]
mod tests {
    use crate::syntax::token::TokenType;

    use super::*;

    fn str_to_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn test_int() {
        let input = str_to_vec("123");
        let start_pos = 0;

        let (token, next) = scan(&input, start_pos);
        assert_eq!(3, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("123", token.value);


        let input = str_to_vec("0123");
        let start_pos = 0;

        let (token, next) = scan(&input, start_pos);
        assert_eq!(4, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("0123", token.value);


        let input = str_to_vec("  123456 789");
        let start_pos = 2;

        let (token, next) = scan(&input, start_pos);
        assert_eq!(8, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("123456", token.value);
    }
}
