use crate::{
    lexic::token::Token,
    syntax::{
        ast::{
            loops::ForLoop, var_binding::VariableBinding, Conditional, FunctionDeclaration,
            Statement,
        },
        parseable::{Parseable, ParsingError, ParsingResult},
    },
};

impl<'a> Parseable<'a> for Statement<'a> {
    type Item = Statement<'a>;

    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        // Try to parse a variable binding
        match VariableBinding::try_parse(tokens, current_pos) {
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
        match FunctionDeclaration::try_parse(tokens, current_pos) {
            Ok((prod, next)) => {
                return Ok((Statement::FnDecl(prod), next));
            }
            Err(ParsingError::Err(error)) => {
                // TODO: Better error handling, write a better error message
                return Err(ParsingError::Err(error));
            }
            _ => {}
        }

        // Try to parse a conditional
        match Conditional::try_parse(tokens, current_pos) {
            Ok((prod, next)) => return Ok((Statement::Conditional(prod), next)),
            Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
            _ => {}
        }

        // Try to parse a for loop
        match ForLoop::try_parse(tokens, current_pos) {
            Ok((prod, next)) => return Ok((Statement::ForLoop(prod), next)),
            Err(ParsingError::Err(e)) => return Err(ParsingError::Err(e)),
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

    #[test]
    fn should_parse_variable_decl() {
        let input = String::from("val x = 322");
        let tokens = get_tokens(&input).unwrap();
        let parsing = Statement::try_parse(&tokens, 0);

        match parsing {
            Ok(_) => {}
            Err(_) => panic!("Expected parsing to be successful"),
        }
    }

    #[test]
    fn should_not_parse_invalid_variable_decl() {
        let input = String::from("val x y");
        let tokens = get_tokens(&input).unwrap();
        let parsing = Statement::try_parse(&tokens, 0);

        match parsing {
            Ok(_) => panic!("Expected an Err"),
            Err(_) => {}
        }
    }

    #[test]
    fn should_parse_fun_decl() {
        let input = String::from("fun name(){}");
        let tokens = get_tokens(&input).unwrap();
        let parsing = Statement::try_parse(&tokens, 0);

        match parsing {
            Ok(_) => {}
            Err(_) => panic!("Expected parsing to be successful"),
        }
    }
}
