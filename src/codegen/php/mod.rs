use super::Transpilable;
use crate::php_ast::PhpExpression;

pub mod statement;
pub mod statement_list;
mod expression;

impl Transpilable for PhpExpression<'_> {
    fn transpile(&self) -> String {
        match self {
            PhpExpression::PrimaryExpression(p) => p.transpile(),
        }
    }
}
