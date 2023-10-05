use crate::lexic::token::Token;

use super::{ast::statement::Statement, functions::function_call, ParseResult};

pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParseResult<Statement, ()> {
    None.or_else(|| match function_call::try_parse(tokens, pos) {
        ParseResult::Ok(f, next) => Some(ParseResult::Ok(Statement::FunctionCall(f), next)),
        ParseResult::Err(err) => Some(ParseResult::Err(err)),
        _ => None,
    })
    .unwrap_or_else(|| ParseResult::Unmatched)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_function_call() {
        let input = String::from("f1()");
        let tokens = crate::lexic::get_tokens(&input).unwrap();
        let statement = try_parse(&tokens, 0);

        let statement = match statement {
            ParseResult::Ok(s, _) => s,
            _ => panic!("Expected a function call"),
        };

        match statement {
            Statement::FunctionCall(_) => assert!(true),
            _ => panic!("Expected a function call"),
        }
    }
}
