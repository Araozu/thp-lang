use crate::{
    error_handling::{error_messages::SYNTAX_INVALID_ASSIGNMENT, ErrorContainer, ErrorLabel},
    lexic::token::{self, TokenType},
    syntax::{
        ast::{Assignment, Expression},
        parseable::{self, Parseable, ParsingError},
        utils::{parse_token_type, try_operator},
    },
};

/// https://thp-lang.org/spec/ast/ast/#assignment
impl<'a> Parseable<'a> for Assignment<'a> {
    type Item = Assignment<'a>;

    fn try_parse(
        tokens: &'a Vec<token::Token>,
        current_pos: usize,
    ) -> parseable::ParsingResult<'a, Self::Item> {
        // parse the target identifier
        let (identifier, next) = match parse_token_type(tokens, current_pos, TokenType::Identifier)
        {
            Ok(tuple) => tuple,
            _ => return Err(ParsingError::Unmatched),
        };

        // parse the equal sign
        let (equal_operator, next) = match try_operator(tokens, next, String::from("=")) {
            Ok((t, next)) => (t, next),
            Err(ParsingError::Mismatch(t)) => {
                // The parser found a token, but it's not the `=` operator
                let label = ErrorLabel {
                    message: String::from("Expected an equal sign `=` here, the identifier"),
                    start: t.position,
                    end: t.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_ASSIGNMENT,
                    error_offset: t.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
            _ => {
                // The parser didn't find the `=` operator after the identifier
                let label = ErrorLabel {
                    message: String::from("Expected an equal sign `=` after this identifier"),
                    start: identifier.position,
                    end: identifier.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_ASSIGNMENT,
                    error_offset: identifier.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };

        // parse the expression
        let (expression, next) = match Expression::try_parse(tokens, next) {
            Ok((exp, next)) => (exp, next),
            _ => {
                let label = ErrorLabel {
                    message: String::from("Expected an expression after this equal `=` operator"),
                    start: equal_operator.position,
                    end: equal_operator.get_end_position(),
                };
                let econtainer = ErrorContainer {
                    error_code: SYNTAX_INVALID_ASSIGNMENT,
                    error_offset: equal_operator.position,
                    labels: vec![label],
                    note: None,
                    help: None,
                };
                return Err(ParsingError::Err(econtainer));
            }
        };

        // Build and return the assignment object
        let assignment = Assignment {
            identifier,
            expression: Box::new(expression),
        };

        Ok((assignment, next))
    }
}
