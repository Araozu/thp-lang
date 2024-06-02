use crate::syntax::{
    ast::Statement,
    binding,
    functions::function_declaration,
    parseable::{Parseable, ParsingError},
};

impl<'a> Parseable<'a> for Statement<'a> {
    type Item = Statement<'a>;

    fn try_parse(
        tokens: &'a Vec<crate::lexic::token::Token>,
        current_pos: usize,
    ) -> crate::syntax::parseable::ParsingResult<'a, Self::Item> {
        // Try to parse a variable binding
        // TODO: Rewrite function_declaration to use Parseable
        match binding::try_parse(tokens, current_pos) {
            Ok((prod, next)) => {
                return Ok((Statement::Binding(prod), next));
            }
            Err(ParsingError::Err(error)) => {
                // TODO: Better error handling, write a better error message
                return Err(ParsingError::Err(error));
            }
            _ => {}
        }

        // Try to parse a function declaration
        // TODO: Rewrite function_declaration to use Parseable
        match function_declaration::try_parse(tokens, current_pos) {
            Ok((prod, next)) => {
                return Ok((Statement::FnDecl(prod), next));
            }
            Err(ParsingError::Err(error)) => {
                // TODO: Better error handling, write a better error message
                return Err(ParsingError::Err(error));
            }
            _ => {}
        }

        // Here nothing was parsed.
        Err(ParsingError::Unmatched)
    }
}

#[cfg(test)]
mod test {
    use crate::lexic::get_tokens;

    use super::*;

    #[test]
    fn should_parse_fn_decl_1() {
        let tokens = get_tokens(&String::from("fun id() {}")).unwrap();

        match Statement::try_parse(&tokens, 0) {
            Ok((prod, next)) => {
                assert_eq!(6, next);
                match prod {
                    Statement::FnDecl(fn_decl) => {
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
    fn should_parse_fn_decl_w_whitespace() {
        let tokens = get_tokens(&String::from("\nfun id() {}")).unwrap();

        match Statement::try_parse(&tokens, 0) {
            Ok((prod, next)) => {
                assert_eq!(7, next);
                match prod {
                    Statement::FnDecl(fn_decl) => {
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
}
