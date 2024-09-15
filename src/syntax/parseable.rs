use crate::{error_handling::ErrorContainer, lexic::token::Token};

/// The result of a parsing operation.
/// On success, it contains the item and the position of the next token
/// On failure, it contains the error
pub type ParsingResult<'a, A> = Result<(A, usize), ParsingError<'a>>;

#[derive(Debug)]
pub enum ParsingError<'a> {
    /// The parsing didn't succeed, but it's not a fatal error
    Unmatched,
    /// Some other token was found than the expected one
    Mismatch(&'a Token),
    /// The parsing failed past a point of no return.
    ///
    /// For example, when parsing a function declaration
    /// the `fun` token is found, but then no identifier
    Err(ErrorContainer),
}

/// Represents a type that can be parsed using Recursive Descent
pub trait Parseable<'a> {
    type Item;

    /// Try to parse the current production.
    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item>;
}
