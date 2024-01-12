use crate::syntax::ast::functions::FunctionCall;

use super::Transpilable;

impl Transpilable for FunctionCall {
    fn transpile(&self) -> String {
        format!("{}()", self.function.transpile())
    }
}
