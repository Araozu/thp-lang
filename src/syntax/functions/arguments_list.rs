use crate::{
    error_handling::{error_messages::SYNTAX_INCOMPLETE_ARGUMENT_LIST, ErrorContainer, ErrorLabel},
    lexic::token::{Token, TokenType},
    syntax::{
        ast::{functions::ArgumentsList, Expression},
        parseable::Parseable,
        utils::parse_token_type,
        ParsingError, ParsingResult,
    },
};

pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParsingResult<ArgumentsList> {
    let mut current_pos = pos;

    let (opening_paren, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::LeftParen) {
            Ok((t, next)) => (t, next),
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(t)) => return Err(ParsingError::Mismatch(t)),
            Err(ParsingError::Unmatched) => return Err(ParsingError::Unmatched),
        };
    current_pos = next_pos;

    let mut arguments = Vec::<Expression>::new();
    loop {
        let (next_expression, next_pos) = match Expression::try_parse(tokens, current_pos) {
            Ok((expression, next_pos)) => (expression, next_pos),
            Err(ParsingError::Err(error)) => {
                // TODO: Write a more detailed error
                return Err(ParsingError::Err(error));
            }
            _ => break,
        };
        current_pos = next_pos;

        arguments.push(next_expression);

        // Parse comma. This also parses a trailing comma
        match parse_token_type(tokens, current_pos, TokenType::Comma) {
            Ok((_, next)) => {
                current_pos = next;
            }
            // This should never happen
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(_)) => {
                // Something other than a comma was found. It must be a closing paren )
                // Still, break the loop, assume there are no more arguments
                // TODO: This could be a good place to write a detailed error?
                break;
            }
            Err(ParsingError::Unmatched) => break,
        };
    }

    // Parse closing paren
    let (closing_paren, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::RightParen) {
            Ok((t, next)) => (t, next),
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(t)) => {
                let label_1 = ErrorLabel {
                    message: String::from("The argument list starts here"),
                    start: opening_paren.position,
                    end: opening_paren.get_end_position(),
                };
                let label = ErrorLabel {
                    message: String::from("Expected a closing paren `)` here"),
                    start: t.position,
                    end: t.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INCOMPLETE_ARGUMENT_LIST,
                    error_offset: t.position,
                    labels: vec![label_1, label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            Err(ParsingError::Unmatched) => {
                let label_1 = ErrorLabel {
                    message: String::from("The argument list starts here"),
                    start: opening_paren.position,
                    end: opening_paren.get_end_position(),
                };
                let label_2 = ErrorLabel {
                    message: String::from("The code ends here without closing the argument list"),
                    start: current_pos,
                    end: current_pos + 1,
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INCOMPLETE_ARGUMENT_LIST,
                    error_offset: current_pos,
                    labels: vec![label_1, label_2],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };
    current_pos = next_pos;

    Ok((
        ArgumentsList {
            arguments,
            paren_open_pos: opening_paren.position,
            paren_close_pos: closing_paren.get_end_position(),
        },
        current_pos,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn should_parse_empty_list() {
        let tokens = get_tokens(&String::from("()")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let Ok((list, next)) = fun_decl else {
            panic!("Expected an unmatched result: {:?}", fun_decl);
        };

        assert_eq!(next, 2);
        assert_eq!(list.arguments.len(), 0);
    }

    #[test]
    fn should_parse_empty_list_with_whitespace() {
        let tokens = get_tokens(&String::from("(  )   ")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let Ok((list, next)) = fun_decl else {
            panic!("Expected a result, got: {:?}", fun_decl);
        };

        assert_eq!(next, 2);
        assert_eq!(list.arguments.len(), 0);
    }

    #[test]
    fn should_parse_empty_list_with_whitespace_2() {
        let tokens = get_tokens(&String::from("(\n    \n)")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let Ok((list, next)) = fun_decl else {
            panic!("Expected a result, got: {:?}", fun_decl);
        };

        assert_eq!(next, 3);
        assert_eq!(list.arguments.len(), 0);
    }

    #[test]
    fn should_parse_one_argument() {
        let tokens = get_tokens(&String::from("(0)")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let Ok((arguments_list, next)) = fun_decl else {
            panic!("Expected a result, got: {:?}", fun_decl);
        };

        assert_eq!(next, 3);
        assert_eq!(arguments_list.arguments.len(), 1);

        let first_argument = arguments_list.arguments.get(0).unwrap();

        let Expression::Int(_) = first_argument else {
            panic!("Expected a number")
        };
    }

    #[test]
    fn should_parse_one_argument_with_trailing_comma() {
        let tokens = get_tokens(&String::from("(0, )")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let Ok((arguments_list, next)) = fun_decl else {
            panic!("Expected a result, got: {:?}", fun_decl);
        };

        assert_eq!(next, 4);
        assert_eq!(arguments_list.arguments.len(), 1);

        let first_argument = arguments_list.arguments.get(0).unwrap();
        let Expression::Int(_) = first_argument else {
            panic!("Expected a number")
        };
    }

    #[test]
    fn should_parse_multiple_arguments() {
        let tokens = get_tokens(&String::from("(\"Hello new world\", 322, )")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let Ok((arguments_list, _next)) = fun_decl else {
            panic!("Expected a result, got: {:?}", fun_decl);
        };

        assert_eq!(arguments_list.arguments.len(), 2);

        let first_argument = arguments_list.arguments.get(0).unwrap();
        let Expression::String(_) = first_argument else {
            panic!("Expected a string")
        };

        let second_argument = arguments_list.arguments.get(1).unwrap();
        let Expression::Int(_) = second_argument else {
            panic!("Expected a number")
        };
    }

    #[test]
    fn should_parse_nested_function_calls() {
        let tokens = get_tokens(&String::from("(foo(), bar())")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let Ok((arguments_list, _next)) = fun_decl else {
            panic!("Expected a result, got: {:?}", fun_decl);
        };

        assert_eq!(arguments_list.arguments.len(), 2);

        let first_argument = arguments_list.arguments.get(0).unwrap();
        let Expression::FunctionCall(_f1) = first_argument else {
            panic!("Expected a function call")
        };

        let second_argument = arguments_list.arguments.get(1).unwrap();
        let Expression::FunctionCall(_) = second_argument else {
            panic!("Expected a number")
        };
    }
}
