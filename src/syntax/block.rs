use crate::{
    error_handling::SyntaxError,
    lexic::token::{Token, TokenType},
};

use super::{ast::Block, utils::parse_token_type, ParsingError, ParsingResult};

// Assumes that the token at `pos` is a {
pub fn parse_block<'a>(tokens: &'a Vec<Token>, pos: usize) -> ParsingResult<Block> {
    let mut current_pos = pos;

    let (opening_brace, next_pos) = parse_token_type(tokens, current_pos, TokenType::LeftBrace)?;
    current_pos = next_pos;

    // Parse block statements
    let mut statements = Vec::new();

    // First statement
    match super::statement::try_parse(tokens, current_pos) {
        Ok((statement, next_pos)) => {
            current_pos = next_pos;
            statements.push(statement);
        }
        Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
        _ => {}
    }

    // More statements separated by new lines
    while let Some(t) = tokens.get(current_pos) {
        if t.token_type != TokenType::NewLine {
            break;
        }
        current_pos += 1;

        match super::statement::try_parse(tokens, current_pos) {
            Ok((statement, next_pos)) => {
                current_pos = next_pos;
                statements.push(statement);
            }
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            _ => break,
        }
    }

    // Parse closing brace
    let (_closing_brace, next_pos) =
        match parse_token_type(tokens, current_pos, TokenType::RightBrace) {
            Ok((t, next)) => (t, next),
            Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
            Err(ParsingError::Mismatch(t)) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected a closing brace after the block body."),
                    error_start: t.position,
                    error_end: t.get_end_position(),
                }));
            }
            Err(ParsingError::Unmatched) => {
                return Err(ParsingError::Err(SyntaxError {
                    reason: String::from("Expected a closing brace after the block body."),
                    error_start: opening_brace.position,
                    error_end: opening_brace.get_end_position(),
                }));
            }
        };
    current_pos = next_pos;

    Ok((Block { statements }, current_pos))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn test_parse_block() {
        let tokens = get_tokens(&String::from("{f()}")).unwrap();
        let block = parse_block(&tokens, 0);

        let block = match block {
            ParsingResult::Ok((p, _)) => p,
            _ => panic!("Expected a block, got: {:?}", block),
        };

        assert_eq!(block.statements.len(), 1);
    }

    #[test]
    fn test_parse_block_2() {
        let tokens = get_tokens(&String::from("{f()\ng()}")).unwrap();
        let block = parse_block(&tokens, 0);

        let block = match block {
            ParsingResult::Ok((p, _)) => p,
            _ => panic!("Expected a block, got: {:?}", block),
        };

        assert_eq!(block.statements.len(), 2);
    }

    #[test]
    fn test_parse_block_3() {
        let tokens = get_tokens(&String::from("{\n    f()\n}")).unwrap();
        let block = parse_block(&tokens, 0);

        let block = match block {
            ParsingResult::Ok((p, _)) => p,
            _ => {
                panic!("Expected a block, got: {:?}\n\n{:?}", block, tokens)
            }
        };

        assert_eq!(block.statements.len(), 1);
    }
}
