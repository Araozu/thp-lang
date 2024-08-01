use super::Transpilable;
use crate::php_ast::PhpExpression;

mod expression;
pub mod statement;
pub mod statement_list;

impl Transpilable for PhpExpression<'_> {
    fn transpile(&self) -> String {
        match self {
            PhpExpression::Assignment(p) => p.transpile(),
        }
    }
}
