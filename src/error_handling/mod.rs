use serde::Serialize;

use self::semantic_error::SemanticError;

mod lex_error;
pub mod semantic_error;
mod syntax_error;
mod utils;

pub trait PrintableError {
    fn get_error_str(&self, chars: &Vec<char>) -> String;
    fn print_ariadne(&self, source: &String);
}

#[derive(Serialize, Debug)]
pub enum MistiError {
    Lex(LexError),
    Syntax(SyntaxError),
    Semantic(SemanticError),
}

#[derive(Serialize, Debug)]
pub struct LexError {
    pub position: usize,
    // TODO: Add and end position
    pub end_position: usize,
    pub reason: String,
}

#[derive(Serialize, Debug)]
pub struct SyntaxError {
    pub error_start: usize,
    pub error_end: usize,
    pub reason: String,
}

impl PrintableError for MistiError {
    fn get_error_str(&self, chars: &Vec<char>) -> String {
        match self {
            Self::Lex(err) => err.get_error_str(chars),
            Self::Syntax(err) => err.get_error_str(chars),
            Self::Semantic(err) => err.get_error_str(chars),
        }
    }

    fn print_ariadne(&self, source: &String) {
        match self {
            Self::Lex(err) => err.print_ariadne(source),
            Self::Syntax(err) => err.print_ariadne(source),
            Self::Semantic(err) => err.print_ariadne(source),
        }
    }
}
