use crate::syntax::{
    ast::Statement, binding, functions::function_declaration, parseable::Parseable,
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
            Err(_) => {
                // TODO
            }
        }

        // Try to parse a function declaration
        // TODO: Rewrite function_declaration to use Parseable
        match function_declaration::try_parse(tokens, current_pos) {
            Ok((prod, next)) => {
                return Ok((Statement::FnDecl(prod), next));
            }
            Err(_) => {
                // TODO
            }
        }

        // Here nothing was parsed. Should fail
        todo!("Nothing was parsed. Should fail")
    }
}
