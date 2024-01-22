use crate::syntax::ast::functions::FunctionCall;

use super::Transpilable;

impl Transpilable for FunctionCall {
    fn transpile(&self) -> String {
        let parameters = &self
            .arguments
            .arguments
            .iter()
            .map(|expr| expr.transpile())
            .collect::<Vec<_>>()
            .join(", ");

        format!("{}({})", self.function.transpile(), parameters)
    }
}
