use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
    syntax::utils::parse_token_type,
};

use super::super::{
    ast::{Parameter, ParamsList},
    utils, ParseResult,
};

pub fn parse_params_list<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
) -> ParseResult<ParamsList, &Token> {
    let mut current_pos = pos;

    let (opening_paren, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::LeftParen) {
            ParseResult::Ok(t, next) => (t, next),
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Mismatch(t) => return ParseResult::Mismatch(t),
            ParseResult::Unmatched => return ParseResult::Unmatched,
        };
    current_pos = next_pos;

    // Parse parameters definitions, separated by commas
    let mut parameters = Vec::<Parameter>::new();
    loop {
        let (next_parameter, next_pos) = match parse_param_definition(tokens, current_pos) {
            ParseResult::Ok(parameter, next_pos) => (parameter, next_pos),
            ParseResult::Err(error) => {
                return ParseResult::Err(error);
            }
            _ => break,
        };
        current_pos = next_pos;
        parameters.push(next_parameter);

        // Parse comma. This also parses a trailing comma
        match parse_token_type(tokens, current_pos, TokenType::Comma) {
            ParseResult::Ok(_, next) => {
                current_pos = next;
            }
            // This should never happen
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Mismatch(_) => {
                // Something other than a comma was found. It must be a closing paren )
                // Still, break the loop, assume there are no more arguments
                // TODO: This could be a good place to write a detailed error?
                break;
            }
            ParseResult::Unmatched => break,
        };
    }

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

    ParseResult::Ok(ParamsList {}, current_pos)
}

fn parse_param_definition<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
) -> ParseResult<Parameter, &Token> {
    // Parse a single parameter definition of the form:
    // - Type identifier
    // There will be more constructs in the future, like:
    // - Type identifier = default_value
    // - FunctionType identifier
    // - Pattern identifier (e.g. Some[String] value)?

    let mut current_pos = pos;
    let (datatype, next_pos) =
        match utils::parse_token_type(tokens, current_pos, TokenType::Datatype) {
            ParseResult::Ok(token, next) => (token, next),
            ParseResult::Err(err) => {
                return ParseResult::Err(err);
            }
            // If there is no datatype this construction doesn't apply.
            // Return a mismatch and let the caller handle it
            ParseResult::Mismatch(t) => return ParseResult::Mismatch(t),
            ParseResult::Unmatched => return ParseResult::Unmatched,
        };
    current_pos = next_pos;

    let (identifier, next_pos) =
        match utils::parse_token_type(tokens, current_pos, TokenType::Identifier) {
            ParseResult::Ok(token, next) => (token, next),
            ParseResult::Err(err) => {
                return ParseResult::Err(err);
            }
            // However, if we fail to parse an identifier, it's an error
            ParseResult::Mismatch(_) => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from("Expected an identifier for the parameter."),
                    error_start: tokens[pos].position,
                    error_end: tokens[pos].get_end_position(),
                });
            }
            ParseResult::Unmatched => {
                return ParseResult::Err(SyntaxError {
                    reason: String::from("Expected an identifier for the parameter."),
                    error_start: tokens[pos].position,
                    error_end: tokens[pos].get_end_position(),
                })
            }
        };

    ParseResult::Ok(
        Parameter {
            identifier: Box::new(identifier.value.clone()),
            datatype: Box::new(datatype.value.clone()),
        },
        next_pos,
    )
}
