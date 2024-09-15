use crate::{
    error_handling::{error_messages::SYNTAX_INCOMPLETE_BLOCK, ErrorContainer, ErrorLabel},
    lexic::token::{Token, TokenType},
    syntax::{
        ast::{Block, BlockMember, Expression, Statement},
        parseable::{Parseable, ParsingError, ParsingResult},
        utils::parse_token_type,
    },
};

impl<'a> Parseable<'a> for Block<'a> {
    type Item = Block<'a>;

    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        let mut current_pos = current_pos;

        let (opening_brace, next_pos) =
            parse_token_type(tokens, current_pos, TokenType::LeftBrace)?;
        current_pos = next_pos;

        // Parse BlockMember = Statement | Expression
        let mut members = Vec::<BlockMember>::new();

        let tokens_len = tokens.len();

        // Minus one because last token is EOF
        while current_pos < tokens_len - 1 {
            // Attempt to parse an statement
            match Statement::try_parse(tokens, current_pos) {
                Ok((prod, next_pos)) => {
                    members.push(BlockMember::Stmt(prod));
                    current_pos = next_pos;
                    continue;
                }
                Err(ParsingError::Err(error)) => {
                    // TODO: Better error handling, write a better error message
                    return Err(ParsingError::Err(error));
                }
                _ => {}
            };

            // Attempt to parse an expression
            match Expression::try_parse(tokens, current_pos) {
                Ok((prod, next_pos)) => {
                    members.push(BlockMember::Expr(prod));
                    current_pos = next_pos;
                    continue;
                }
                Err(ParsingError::Err(error)) => {
                    // TODO: Better error handling, write a better error message
                    return Err(ParsingError::Err(error));
                }
                _ => {}
            }

            // If we reached this point we didn't match any productions
            // we get out of the block, the parsing of "}" will deal
            // with any incorrect tokens
            break;
        }

        // Parse closing brace
        let (closing_brace, next_pos) =
            match parse_token_type(tokens, current_pos, TokenType::RightBrace) {
                Ok((t, next)) => (t, next),
                Err(ParsingError::Err(err)) => return Err(ParsingError::Err(err)),
                Err(ParsingError::Mismatch(t)) => {
                    let label = ErrorLabel {
                        message: String::from("Expected a closing brace `}` here"),
                        start: t.position,
                        end: t.get_end_position(),
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INCOMPLETE_BLOCK,
                        error_offset: t.position,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
                }
                Err(ParsingError::Unmatched) => {
                    let label = ErrorLabel {
                        message: String::from("Expected a closing brace `}` here"),
                        start: current_pos,
                        end: current_pos + 1,
                    };
                    let econtainer = ErrorContainer {
                        error_code: SYNTAX_INCOMPLETE_BLOCK,
                        error_offset: current_pos,
                        labels: vec![label],
                        note: None,
                        help: None,
                    };
                    return Err(ParsingError::Err(econtainer));
                }
            };
        current_pos = next_pos;

        let block = Block {
            members,
            start: opening_brace.position,
            end: closing_brace.position,
        };
        Ok((block, current_pos))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexic::get_tokens;

    #[test]
    fn should_parse_empty_block() {
        let tokens = get_tokens(&String::from("{}")).unwrap();
        let (block, next_pos) = Block::try_parse(&tokens, 0).unwrap();

        assert_eq!(2, next_pos);
        assert_eq!(0, block.members.len())
    }

    #[test]
    fn should_parse_block_with_fn() {
        let tokens = get_tokens(&String::from("{\n    fun f(){}\n}")).unwrap();
        let (block, next_pos) = Block::try_parse(&tokens, 0).unwrap();

        assert_eq!(10, next_pos);
        assert_eq!(1, block.members.len());

        let member = &block.members[0];
        match member {
            BlockMember::Stmt(Statement::FnDecl(f)) => {
                assert_eq!(f.identifier.value, "f");
            }
            _ => panic!("Expected a function declaration, got {:?}", member),
        }
    }

    #[test]
    fn should_parse_block_with_fn_2() {
        let tokens = get_tokens(&String::from("{\n    fun f(){}\nfun g(){}\n}")).unwrap();
        let (block, next_pos) = Block::try_parse(&tokens, 0).unwrap();

        assert_eq!(17, next_pos);
        assert_eq!(2, block.members.len());

        let member = &block.members[0];
        match member {
            BlockMember::Stmt(Statement::FnDecl(f)) => {
                assert_eq!(f.identifier.value, "f");
            }
            _ => panic!("Expected a function declaration, got {:?}", member),
        }

        let member = &block.members[1];
        match member {
            BlockMember::Stmt(Statement::FnDecl(f)) => {
                assert_eq!(f.identifier.value, "g");
            }
            _ => panic!("Expected a function declaration, got {:?}", member),
        }
    }

    // TODO: rewrite, refactor
    #[test]
    fn should_parse_simple_expression() {
        let tokens = get_tokens(&String::from("{f()}")).unwrap();
        let (block, _) = Block::try_parse(&tokens, 0).unwrap();

        assert_eq!(block.members.len(), 1);
    }

    #[test]
    fn test_parse_block_2() {
        let tokens = get_tokens(&String::from("{f()\ng()}")).unwrap();
        let block = Block::try_parse(&tokens, 0);

        let block = match block {
            ParsingResult::Ok((p, _)) => p,
            _ => panic!("Expected a block, got: {:?}", block),
        };

        assert_eq!(block.members.len(), 2);
    }

    #[test]
    fn test_parse_block_3() {
        let tokens = get_tokens(&String::from("{\n    f()\n}")).unwrap();
        let block = Block::try_parse(&tokens, 0);

        let block = match block {
            ParsingResult::Ok((p, _)) => p,
            _ => {
                panic!("Expected a block, got: {:?}\n\n{:?}", block, tokens)
            }
        };

        assert_eq!(block.members.len(), 1);
    }
}
