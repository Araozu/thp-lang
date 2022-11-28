use super::token::Token;

mod number;

pub fn number(chars: &Vec<char>, start_pos: usize) -> Result<(Token, usize), String> {
    number::scan(chars, start_pos)
}

