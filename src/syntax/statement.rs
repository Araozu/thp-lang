use crate::lexic::token::Token;

use super::{
    ast::{statement::Statement, Expression},
    binding,
    expression::function_call_expr,
    ParsingError, ParsingResult,
};

pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParsingResult<Statement> {
    // Try to parse a binding
    match binding::try_parse(tokens, pos) {
        Ok((b, next)) => return Ok((Statement::Binding(b), next)),
        Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
        _ => {}
    }

    // Try to parse a function call
    match function_call_expr::try_parse(tokens, pos) {
        Ok((Expression::FunctionCall(f), next)) => return Ok((Statement::FunctionCall(f), next)),
        Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
        _ => {}
    };

    // Return unmatched
    Err(ParsingError::Unmatched)
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
            Ok((s, _)) => s,
            _ => panic!("Expected a statement"),
        };

        match statement {
            Statement::FunctionCall(_) => assert!(true),
            _ => panic!("Expected a function call"),
        }
    }

    #[test]
    fn should_parse_binding() {
        let input = String::from("let identifier = 20");
        let tokens = crate::lexic::get_tokens(&input).unwrap();
        let statement = try_parse(&tokens, 0);

        let statement = match statement {
            Ok((s, _)) => s,
            _ => panic!("Expected a statement"),
        };

        match statement {
            Statement::Binding(_) => assert!(true),
            _ => panic!("Expected a binding"),
        }
    }
}
