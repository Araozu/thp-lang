use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
    syntax::{ast::functions::ArgumentsList, utils::parse_token_type, ParseResult},
};

pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParseResult<ArgumentsList, &Token> {
    let mut current_pos = pos;

    let (opening_paren, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::LeftParen) {
            ParseResult::Ok(t, next) => (t, next),
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Mismatch(t) => return ParseResult::Mismatch(t),
            ParseResult::Unmatched => return ParseResult::Unmatched,
        };
    current_pos = next_pos;

    // Parse closing paren
    let (_closing_paren, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::RightParen) {
            ParseResult::Ok(t, next) => (t, next),
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Mismatch(t) => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from("Expected a closing paren after the function identifier."),
                    error_start: t.position,
                    error_end: t.get_end_position(),
                });
            }
            ParseResult::Unmatched => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from("Expected a closing paren after the function identifier."),
                    error_start: opening_paren.position,
                    error_end: opening_paren.get_end_position(),
                });
            }
        };
    current_pos = next_pos;

    ParseResult::Ok(
        ArgumentsList {
            arguments: Vec::new(),
        },
        current_pos,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn should_parse_empty_list() {
        let tokens = get_tokens(&String::from("()")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let ParseResult::Ok(list, next) = fun_decl else {
            panic!("Expected an unmatched result: {:?}", fun_decl);
        };

        assert_eq!(next, 2);
        assert_eq!(list.arguments.len(), 0);
    }

    #[test]
    fn should_parse_empty_list_with_whitespace() {
        let tokens = get_tokens(&String::from("(  )   ")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let ParseResult::Ok(list, next) = fun_decl else {
            panic!("Expected a result, got: {:?}", fun_decl);
        };

        assert_eq!(next, 2);
        assert_eq!(list.arguments.len(), 0);
    }

    #[test]
    fn should_parse_empty_list_with_whitespace_2() {
        let tokens = get_tokens(&String::from("(\n    \n)")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let ParseResult::Ok(list, next) = fun_decl else {
            panic!("Expected a result, got: {:?}", fun_decl);
        };

        assert_eq!(next, 3);
        assert_eq!(list.arguments.len(), 0);
    }
}
