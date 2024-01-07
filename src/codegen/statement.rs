use crate::syntax::ast::statement::Statement;

use super::Transpilable;

impl Transpilable for Statement {
    fn transpile(&self) -> String {
<<<<<<< HEAD
        match self {
            Statement::Binding(binding) => binding.transpile(),
            Statement::FunctionCall(function_call) => function_call.transpile(),
        }
=======
        let stmt = match self {
            Statement::FunctionCall(f) => f.transpile(),
            Statement::Binding(b) => b.transpile(),
        };

        format!("{stmt};")
>>>>>>> f71f9ab ((lazily) codegen parsed expressions. v0.0.9)
    }
}
