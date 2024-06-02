use crate::{
    lexic::token::Token,
    syntax::{
        ast::Expression,
        parseable::{Parseable, ParsingResult},
    },
};

impl<'a> Parseable<'a> for Expression<'a> {
    type Item = Expression<'a>;

    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        todo!()
    }
}
