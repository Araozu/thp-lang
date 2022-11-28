use crate::syntax::{token::{Token, self}, utils};

pub fn scan(chars: &Vec<char>, start_pos: usize) -> (Token, usize) {
    let next_char = chars.get(start_pos);

    // Try to scan a HEX value
    if let Some(c) = next_char {
        if *c == '0' {
            // May be an 'x' or 'X'
            let next_next_char = chars.get(start_pos + 1);
            if let Some(c2) = next_next_char {
                if *c2 == 'x' || *c2 == 'X' {
                    return scan_hex(chars, start_pos + 2, String::from("0x"));
                }
            }
        }
    }


    scan_decimal(chars, start_pos, String::from(""))
}

/// Scans an integer.
fn scan_decimal(chars: &Vec<char>, start_pos: usize, current: String) -> (Token, usize) {
    let next_char = chars.get(start_pos);

    // If a char is found
    if let Some(c) = next_char {
        let c = *c;

        // Scan a decimal number
        if utils::is_digit(c) {
            let new_value = format!("{}{}", current, c);
            return scan_decimal(chars, start_pos + 1, new_value)
        }
   
    }

    // Return the current value
    (token::new_number(current, start_pos as i32), start_pos)
}

/// Scans a hex number. If successful, always returns '0x...', never '0X...'
/// 
/// `current == ""`
/// 
/// `start_pos` indicates the start of the hex value
fn scan_hex(chars: &Vec<char>, start_pos: usize, current: String) -> (Token, usize) {
    let next_char = chars.get(start_pos);

    if let Some(c) = next_char {
        let c = *c;

        if utils::is_hex_digit(c) {
            return scan_hex(chars, start_pos + 1, utils::str_append(current, c))
        }
    }

    // Return current value
    (token::new_number(current, start_pos as i32), start_pos)
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

    #[test]
    fn test_hex() {
        let input = str_to_vec("0x20");
        let start_pos = 0;

        let (token, next) = scan(&input, start_pos);
        assert_eq!(4, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("0x20", token.value);


        let input = str_to_vec("    0Xff23DA");
        let start_pos = 4;

        let (token, next) = scan(&input, start_pos);
        assert_eq!(12, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("0xff23DA", token.value);
    }

    // Should not scan an incomplete hex value
    #[test]
    fn test_hex_2() {
        let input = str_to_vec("0x20");
        let start_pos = 0;

        let (token, next) = scan(&input, start_pos);
        assert_eq!(4, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("0x20", token.value);
    }
}
