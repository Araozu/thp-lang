use crate::lexic::token::Token;

use super::{Block, Expression};

#[derive(Debug)]
pub struct ForLoop<'a> {
    /// the start position of the
    /// `for` keyword
    pub loop_start: usize,
    /// the position of the closing bracket
    pub loop_end: usize,
    pub key: Option<&'a Token>,
    pub value: &'a Token,
    pub collection: Expression<'a>,
    pub body: Block<'a>,
}

#[derive(Debug)]
pub struct WhileLoop<'a> {
    /// the start position of the
    /// `for` keyword
    pub loop_start: usize,
    /// the position of the closing bracket
    pub loop_end: usize,
    pub condition: Expression<'a>,
    pub body: Block<'a>,
}
