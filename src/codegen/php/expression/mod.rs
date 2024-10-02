use crate::{codegen::Transpilable, php_ast::PExpresssion};
use PExpresssion::*;

mod assignment;
mod primary_expression;

impl Transpilable for PExpresssion<'_> {
    fn transpile(&self) -> String {
        match self {
            Primary(p) => p.transpile(),
            Assignment(a) => a.transpile(),
            FunctionCall(f) => f.transpile(),
            BinaryOp(left, right, op) => {
                let left_str = left.transpile();
                let right_str = right.transpile();
                format!("{} {} {}", left_str, op, right_str)
            }
        }
    }
}
