use crate::syntax::ast::functions::FunctionCall;

use super::Transpilable;

impl Transpilable for FunctionCall {
    fn transpile(&self) -> String {
<<<<<<< HEAD
        format!("{}();", self.function.transpile())
=======
        format!("{}()", self.identifier)
>>>>>>> f71f9ab ((lazily) codegen parsed expressions. v0.0.9)
    }
}
