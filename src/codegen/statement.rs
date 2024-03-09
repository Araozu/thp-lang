use crate::syntax::ast::statement::Statement;

use super::Transpilable;

impl Transpilable for Statement<'_> {
    fn transpile(&self) -> String {
        let stmt = match self {
            Statement::FunctionCall(f) => f.transpile(),
            Statement::Binding(b) => b.transpile(),
        };

        format!("{stmt};")
    }
}
