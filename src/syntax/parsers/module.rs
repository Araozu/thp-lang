use crate::{
    error_handling::{
        error_messages::{SYNTAX_INCOMPLETE_STATEMENT, SYNTAX_UNEXPECTED_TOKENS},
        ErrorContainer, ErrorLabel,
    },
    lexic::token::{Token, TokenType},
    syntax::{
        ast::{Expression, ModuleAST, ModuleMembers, Statement},
        parseable::{Parseable, ParsingError, ParsingResult},
        utils::parse_terminator,
    },
};

impl<'a> Parseable<'a> for ModuleAST<'a> {
    type Item = ModuleAST<'a>;

    /// Parses a THP module/source file
    ///
    /// As this function parses the whole file, it ignores `current_pos` and
    /// always starts from token 0.
    ///
    /// Its grammar is defined at the spec, at the webpage
    fn try_parse(tokens: &'a Vec<Token>, _current_pos: usize) -> ParsingResult<'a, Self::Item> {
        let mut productions = Vec::<ModuleMembers>::new();
        let tokens_len = tokens.len();
        let mut current_pos = 0;

        // Minus one because last token is EOF
        while current_pos < tokens_len - 1 {
            // Attempt to parse an statement
            match Statement::try_parse(tokens, current_pos) {
                Ok((prod, next_pos)) => {
                    productions.push(ModuleMembers::Stmt(prod));
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
                    // After a expression is parsed as an statement
                    // there should be a delimiter (new line or EOF)
                    let next_pos = match parse_terminator(tokens, next_pos) {
                        Ok((_, next)) => next,
                        Err(ParsingError::Mismatch(t)) => {
                            let label = ErrorLabel {
                                message: String::from(
                                    "Expected a new line here, found another token",
                                ),
                                start: t.position,
                                end: t.get_end_position(),
                            };
                            let econtainer = ErrorContainer {
                                error_code: SYNTAX_INCOMPLETE_STATEMENT,
                                error_offset: t.position,
                                labels: vec![label],
                                note: Some(String::from(
                                    "There may only be one statement per line",
                                )),
                                help: None,
                            };
                            return Err(ParsingError::Err(econtainer));
                        }
                        _ => unreachable!(),
                    };

                    productions.push(ModuleMembers::Expr(prod));
                    current_pos = next_pos;
                    continue;
                }
                Err(ParsingError::Err(error)) => {
                    // TODO: Better error handling, write a better error message
                    return Err(ParsingError::Err(error));
                }
                _ => {}
            }

            // Ignore comments, if any
            if let Some(s) = tokens.get(current_pos) {
                if s.token_type == TokenType::Comment || s.token_type == TokenType::MultilineComment
                {
                    current_pos += 1;
                    continue;
                }
            }

            // If we reached this point we didn't match any productions and should fail
            let t = &tokens[current_pos];

            let label = ErrorLabel {
                message: String::from("This sequence of tokens couldn't be parsed"),
                start: t.position,
                end: t.get_end_position(),
            };
            let econtainer = ErrorContainer {
                error_code: SYNTAX_UNEXPECTED_TOKENS,
                error_offset: t.position,
                labels: vec![label],
                note: None,
                help: None,
            };
            return Err(ParsingError::Err(econtainer));
        }

        Ok((ModuleAST { productions }, current_pos))
    }
}

#[cfg(test)]
mod test {
    use crate::lexic::get_tokens;

    use super::*;

    #[test]
    fn should_parse_fn_decl_1() {
        let tokens = get_tokens(&String::from("fun id() {}")).unwrap();

        match ModuleAST::try_parse(&tokens, 0) {
            Ok((prods, next)) => {
                assert_eq!(6, next);
                assert_eq!(1, prods.productions.len());

                let prod = &prods.productions[0];
                match prod {
                    ModuleMembers::Stmt(Statement::FnDecl(fn_decl)) => {
                        assert_eq!("id", fn_decl.identifier.value)
                    }
                    _ => panic!("Expected a function declaration"),
                }
            }
            _ => {
                panic!("Expected a function declaration");
            }
        }
    }

    #[test]
    fn should_parse_expression() {
        let tokens = get_tokens(&String::from("1")).unwrap();

        let (_, next) = ModuleAST::try_parse(&tokens, 0).unwrap();

        assert_eq!(next, 2);
    }

    #[test]
    fn should_fail_on_invalid_expression() {
        let tokens = get_tokens(&String::from("function_call(1 2")).unwrap();
        let result = ModuleAST::try_parse(&tokens, 0);

        match result {
            Ok(_) => panic!("Expected an error"),
            Err(_) => {}
        }
    }

    #[test]
    fn should_fail_on_two_expressions_together() {
        // TODO: enforce newlines after every statement
        let tokens = get_tokens(&String::from("print print")).unwrap();
        let result = ModuleAST::try_parse(&tokens, 0);

        match result {
            Ok(_) => panic!("Expected an error"),
            Err(ParsingError::Err(err)) => {
                assert_eq!(err.error_code, SYNTAX_INCOMPLETE_STATEMENT)
            }
            _ => panic!("Expected a parsing error"),
        }
    }
}
