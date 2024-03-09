use self::semantic_error::SemanticError;

mod lex_error;
pub mod semantic_error;
mod syntax_error;
mod utils;

pub trait PrintableError {
    fn get_error_str(&self, chars: &Vec<char>) -> String;
}

#[derive(Debug)]
pub enum MistiError {
    Lex(LexError),
    Syntax(SyntaxError),
    Semantic(SemanticError),
}

#[derive(Debug)]
pub struct LexError {
    pub position: usize,
    pub reason: String,
}

#[derive(Debug)]
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
}
