use crate::{lexic::token::{Token, TokenType}, syntax::{ParseResult, ast::functions::FunctionCall, utils::parse_token_type}, error_handling::SyntaxError};



pub fn try_parse<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParseResult<FunctionCall, ()> {
    let mut current_pos = pos;

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

    ParseResult::Unmatched
}


#[cfg(test)]
mod tests {
    use crate::lexic::get_tokens;
    use super::*;

    #[test]
    fn should_not_parse_identifier_alone() {
        let tokens = get_tokens(&String::from("function_name")).unwrap();
        let fun_decl = try_parse(&tokens, 0);

        let ParseResult::Unmatched = fun_decl else {
            panic!("Expected an unmatched result: {:?}", fun_decl);
        };
    }
}
