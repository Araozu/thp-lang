use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
    syntax::{utils::parse_token_type, ParsingError, ParsingResult},
};

use super::super::{
    ast::{Parameter, ParamsList},
    utils,
};

/*
# Basically, every parameter can have a trailing comma.
params list = "("
            , ( datatype pair, (",", datatype pair)*, ","? )?
            , ")";

datatype pair = datatype, identifier;
 */
pub fn parse_params_list<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParsingResult<ParamsList> {
    let mut current_pos = pos;

    let (opening_paren, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::LeftParen) {
            Ok((t, next)) => (t, next),
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(t)) => return Err(ParsingError::Mismatch(&t)),
            Err(ParsingError::Unmatched) => return Err(ParsingError::Unmatched),
        };
    current_pos = next_pos;

    // Parse parameters definitions, separated by commas
    let mut parameters = Vec::<Parameter>::new();
    loop {
        let (next_parameter, next_pos) = match parse_param_definition(tokens, current_pos) {
            Ok((parameter, next_pos)) => (parameter, next_pos),
            Err(ParsingError::Err(error)) => {
                return Err(ParsingError::Err(error));
            }
            _ => break,
        };
        current_pos = next_pos;
        parameters.push(next_parameter);

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
    let (_closing_paren, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::RightParen) {
            Ok((t, next)) => (t, next),
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(t)) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected a closing paren after the function identifier."),
                    error_start: t.position,
                    error_end: t.get_end_position(),
                }));
            }
            Err(ParsingError::Unmatched) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected a closing paren after the function identifier."),
                    error_start: opening_paren.position,
                    error_end: opening_paren.get_end_position(),
                }));
            }
        };
    current_pos = next_pos;

    Ok((ParamsList { parameters }, current_pos))
}

/// Parse a single parameter definition of the form:
/// - `Type identifier`
///
/// There will be more constructs in the future, like:
/// - `Type identifier = default_value`
/// - `FunctionType identifier`
/// - `Pattern identifier` (e.g. `Some[String] value`)?
fn parse_param_definition<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParsingResult<Parameter> {
    let mut current_pos = pos;
    let (datatype, next_pos) =
        match utils::parse_token_type(tokens, current_pos, TokenType::Datatype) {
            Ok((token, next)) => (token, next),
            Err(ParsingError::Err(err)) => {
                return Err(ParsingError::Err(err));
            }
            // If there is no datatype this construction doesn't apply.
            // Return an unmatch and let the caller handle it
            _ => return Err(ParsingError::Unmatched),
        };
    current_pos = next_pos;

    let (identifier, next_pos) =
        match utils::parse_token_type(tokens, current_pos, TokenType::Identifier) {
            Ok((token, next)) => (token, next),
            Err(ParsingError::Err(err)) => {
                return Err(ParsingError::Err(err));
            }
            // However, if we fail to parse an identifier, it's an error
            Err(ParsingError::Mismatch(_)) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected an identifier for the parameter."),
                    error_start: tokens[pos].position,
                    error_end: tokens[pos].get_end_position(),
                }));
            }
            Err(ParsingError::Unmatched) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected an identifier for the parameter."),
                    error_start: tokens[pos].position,
                    error_end: tokens[pos].get_end_position(),
                }))
            }
        };

    Ok((
        Parameter {
            identifier: &identifier.value,
            datatype: &datatype.value,
        },
        next_pos,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn should_parse_empty_param_list() {
        let tokens = get_tokens(&String::from("()")).unwrap();
        let (result, next_pos) = parse_params_list(&tokens, 0).unwrap();

        assert_eq!(next_pos, 2);
        assert_eq!(result.parameters.len(), 0);
    }

    #[test]
    fn should_parse_empty_param_list_with_whitespace() {
        let tokens = get_tokens(&String::from("(   )")).unwrap();
        let (result, next_pos) = parse_params_list(&tokens, 0).unwrap();

        assert_eq!(next_pos, 2);
        assert_eq!(result.parameters.len(), 0);
    }

    #[test]
    fn should_parse_empty_param_list_with_newlines() {
        let tokens = get_tokens(&String::from("(\n   \n)")).unwrap();
        let (result, next_pos) = parse_params_list(&tokens, 0).unwrap();

        assert_eq!(next_pos, 3);
        assert_eq!(result.parameters.len(), 0);
    }

    #[test]
    fn should_parse_empty_param_list_with_1_parameter() {
        let tokens = get_tokens(&String::from("(Int x)")).unwrap();
        let (result, next_pos) = parse_params_list(&tokens, 0).unwrap();

        assert_eq!(next_pos, 4);
        assert_eq!(result.parameters.len(), 1);
        let first_param = &result.parameters[0];
        assert_eq!(first_param.datatype, "Int");
        assert_eq!(first_param.identifier, "x");
    }

    #[test]
    fn should_parse_empty_param_list_with_1_parameter_with_trailing_comma() {
        let tokens = get_tokens(&String::from("(Int x, )")).unwrap();
        let (result, next_pos) = parse_params_list(&tokens, 0).unwrap();

        assert_eq!(next_pos, 5);
        assert_eq!(result.parameters.len(), 1);
        let first_param = &result.parameters[0];
        assert_eq!(first_param.datatype, "Int");
        assert_eq!(first_param.identifier, "x");
    }

    #[test]
    fn should_parse_empty_param_list_with_2_parameters() {
        let tokens = get_tokens(&String::from("(Int x, String y)")).unwrap();
        let (result, next_pos) = parse_params_list(&tokens, 0).unwrap();

        assert_eq!(next_pos, 7);
        assert_eq!(result.parameters.len(), 2);
        let first_param = &result.parameters[0];
        assert_eq!(first_param.datatype, "Int");
        assert_eq!(first_param.identifier, "x");
        let second_param = &result.parameters[1];
        assert_eq!(second_param.datatype, "String");
        assert_eq!(second_param.identifier, "y");
    }

    #[test]
    fn should_parse_empty_param_list_with_2_parameters_and_trailing_comma() {
        let tokens = get_tokens(&String::from("(Int x, String y, )")).unwrap();
        let (result, next_pos) = parse_params_list(&tokens, 0).unwrap();

        assert_eq!(next_pos, 8);
        assert_eq!(result.parameters.len(), 2);
        let first_param = &result.parameters[0];
        assert_eq!(first_param.datatype, "Int");
        assert_eq!(first_param.identifier, "x");
        let second_param = &result.parameters[1];
        assert_eq!(second_param.datatype, "String");
        assert_eq!(second_param.identifier, "y");
    }

    #[test]
    fn should_parse_multiline_params() {
        let tokens = get_tokens(&String::from("(\n    Int x,\n    String y,\n)")).unwrap();
        let (result, next_pos) = parse_params_list(&tokens, 0).unwrap();

        assert_eq!(next_pos, 11);
        assert_eq!(result.parameters.len(), 2);
        let first_param = &result.parameters[0];
        assert_eq!(first_param.datatype, "Int");
        assert_eq!(first_param.identifier, "x");
        let second_param = &result.parameters[1];
        assert_eq!(second_param.datatype, "String");
        assert_eq!(second_param.identifier, "y");
    }
}
