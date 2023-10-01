use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
    syntax::{ast::functions::FunctionCall, utils::parse_token_type, ParseResult},
};

pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParseResult<FunctionCall, ()> {
    let mut current_pos = pos;

    // TODO: Use an expression instead of a fixed identifier
    // Parse identifier
    let (identifier, next_pos) = match parse_token_type(tokens, current_pos, TokenType::Identifier)
    {
        ParseResult::Ok(id, next) => (id, next),
        ParseResult::Err(err) => return ParseResult::Err(err),
        ParseResult::Mismatch(_) => {
            return ParseResult::Unmatched;
        }
        ParseResult::Unmatched => {
            return ParseResult::Unmatched;
        }
    };
    current_pos = next_pos;

    // Parse arguments list
    let (args_list, next_pos) = match super::arguments_list::try_parse(tokens, current_pos) {
        ParseResult::Ok(args, next) => (args, next),
        ParseResult::Err(err) => return ParseResult::Err(err),
        ParseResult::Mismatch(_) => {
            return ParseResult::Unmatched;
        }
        ParseResult::Unmatched => {
            return ParseResult::Unmatched;
        }
    };
    current_pos = next_pos;

    ParseResult::Ok(
        FunctionCall {
            identifier: Box::new(identifier.value.clone()),
        },
        current_pos,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn should_not_parse_identifier_alone() {
        let tokens = get_tokens(&String::from("function_name")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let ParseResult::Unmatched = fun_decl else {
            panic!("Expected an unmatched result: {:?}", fun_decl);
        };
    }

    #[test]
    fn should_parse_minimal_construct() {
        let tokens = get_tokens(&String::from("function_name()")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let ParseResult::Ok(_call, next) = fun_decl else {
            panic!("Expected a result, got: {:?}", fun_decl);
        };

        assert_eq!(next, 3);
    }

    #[test]
    fn should_parse_minimal_construct_2() {
        let tokens = get_tokens(&String::from("function_name   (    )")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let ParseResult::Ok(_call, next) = fun_decl else {
            panic!("Expected a result, got: {:?}", fun_decl);
        };

        assert_eq!(next, 3);
    }

    #[test]
    fn should_parse_minimal_construct_3() {
        let tokens = get_tokens(&String::from("function_name\n(\n    \n)")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let ParseResult::Ok(_call, next) = fun_decl else {
            panic!("Expected a result, got: {:?}", fun_decl);
        };

        assert_eq!(next, 5);
    }
}
