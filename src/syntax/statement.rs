use crate::lexic::token::Token;

use super::{ast::statement::Statement, functions::function_call, ParseResult, binding};

pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParseResult<Statement, ()> {
    None
        .or_else(
            || match binding::try_parse(tokens, pos) {
                ParseResult::Ok(b, next) => Some(ParseResult::Ok(Statement::Binding(b), next)),
                ParseResult::Err(err) => Some(ParseResult::Err(err)),
                _ => None,
            }
        )
        .or_else(|| match function_call::try_parse(tokens, pos) {
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
            _ => panic!("Expected a statement"),
        };

        match statement {
            Statement::FunctionCall(_) => assert!(true),
            _ => panic!("Expected a function call"),
        }
    }


    #[test]
    fn should_parse_binding() {
        let input = String::from("val identifier = 20");
        let tokens = crate::lexic::get_tokens(&input).unwrap();
        let statement = try_parse(&tokens, 0);

        let statement = match statement {
            ParseResult::Ok(s, _) => s,
            _ => panic!("Expected a statement"),
        };

        match statement {
            Statement::Binding(_) => assert!(true),
            _ => panic!("Expected a binding"),
        }
    }
}
