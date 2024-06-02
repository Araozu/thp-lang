use crate::syntax::{ast::Statement, parseable::Parseable};

impl<'a> Parseable<'a> for Statement<'a> {
    type Item = Statement<'a>;

    fn try_parse(
        tokens: &'a Vec<crate::lexic::token::Token>,
        current_pos: usize,
    ) -> crate::syntax::parseable::ParsingResult<'a, Self::Item> {
        todo!()
    }
}
