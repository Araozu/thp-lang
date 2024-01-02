use crate::syntax::ast::statement::Statement;

use super::Transpilable;

impl Transpilable for Statement {
    fn transpile(&self) -> String {
        match self {
            Statement::Binding(binding) => binding.transpile(),
            Statement::FunctionCall(function_call) => function_call.transpile(),
        }
    }
}
