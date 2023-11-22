use crate::{
    lexic::token::Token,
    syntax::{ast::Expression, ParseResult},
};

pub fn try_parse(tokens: &Vec<Token>, pos: usize) -> ParseResult<Expression, ()> {
    return super::unary::try_parse(tokens, pos);
}
