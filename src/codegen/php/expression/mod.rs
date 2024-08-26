use crate::{codegen::Transpilable, php_ast::php_ast_2::PExpresssion};
use PExpresssion::*;

mod assignment;
mod primary_expression;

impl Transpilable for PExpresssion<'_> {
    fn transpile(&self) -> String {
        match self {
            Primary(p) => p.transpile(),
            Assignment(a) => a.transpile(),
            FunctionCall(f) => f.transpile(),
        }
    }
}
