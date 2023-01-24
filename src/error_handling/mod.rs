mod lex_error;

pub trait PrintableError {
    fn get_error_str(&self, chars: &Vec<char>) -> String;
}

#[derive(Debug)]
pub enum MistiError {
    Lex(LexError)
}

#[derive(Debug)]
pub struct LexError {
    pub position: usize,
    pub reason: String,
}


impl PrintableError for MistiError {
    fn get_error_str(&self, chars: &Vec<char>) -> String {
        match self {
            Self::Lex(err) => err.get_error_str(chars)
        }
    }
}


