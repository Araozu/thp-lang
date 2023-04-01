
mod lexic;
mod token;
mod error_handling;

use token::Token;
use error_handling::MistiError;

pub use token::TokenType;

pub fn tokenize(input: &String) -> Result<Vec<Token>, MistiError> {
    lexic::get_tokens(input)
}
