use super::token::Token;

mod number;
mod operator;

pub fn number(chars: &Vec<char>, start_pos: usize) -> Result<(Token, usize), String> {
    number::scan(chars, start_pos)
}

pub fn operator(chars: &Vec<char>, start_pos: usize) -> (Token, usize) {
    operator::scan(chars, start_pos)
}
