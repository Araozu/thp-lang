use super::token::Token;

mod number;

pub fn number(chars: &Vec<char>, start_pos: usize) -> (Token, usize) {
    number::scan(chars, start_pos)
}

