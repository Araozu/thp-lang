use crate::lexic::token::Token;

use super::Expression;

#[derive(Debug)]
pub struct Binding<'a> {
    pub datatype: Option<&'a Token>,
    pub identifier: &'a Token,
    pub expression: Expression<'a>,
    pub is_mutable: bool,
}
