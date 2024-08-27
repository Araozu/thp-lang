use crate::{
    lexic::token::{Token, TokenType},
    syntax::{
        ast::{Array, Expression},
        parseable::{Parseable, ParsingError, ParsingResult},
        utils::parse_token_type,
    },
};

impl<'a> Parseable<'a> for Array<'a> {
    type Item = Array<'a>;

    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        // parse open bracket
        let (open_bracket, next) =
            match parse_token_type(tokens, current_pos, TokenType::LeftBracket) {
                Ok(t) => t,
                Err(_) => return Err(ParsingError::Unmatched),
            };

        // parse expressions
        let mut exps = Vec::new();
        let mut current_pos = next;
        let tokens_len = tokens.len();
        while current_pos < tokens_len {
            // parse expression
            let (exp, after_exp) = match Expression::try_parse(tokens, current_pos) {
                Ok(t) => t,
                Err(ParsingError::Mismatch(_)) => break,
                Err(ParsingError::Unmatched) => break,
                // If an error is found parsing an exp, bubble up
                Err(e) => return Err(e),
            };

            // add exp to vec
            exps.push(exp);

            // parse comma
            let (_, after_comma) = match parse_token_type(tokens, after_exp, TokenType::Comma) {
                Ok(t) => t,
                // If a comma is not found then the expressions are over
                Err(_) => {
                    current_pos = after_exp;
                    break;
                }
            };

            // update position tracker
            current_pos = after_comma;
        }

        // parse closed bracket
        let (closed_bracket, next) =
            match parse_token_type(tokens, current_pos, TokenType::RightBracket) {
                Ok(t) => t,
                Err(e) => return Err(e),
            };

        // return
        let arr = Array {
            exps,
            start: open_bracket.position,
            end: closed_bracket.position,
        };
        Ok((arr, next))
    }
}
