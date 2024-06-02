use crate::syntax::ast::Statement;

use super::Transpilable;

impl Transpilable for Statement<'_> {
    fn transpile(&self) -> String {
        let stmt = match self {
            Statement::FnDecl(f) => f.transpile(),
            Statement::Binding(b) => b.transpile(),
        };

        format!("{stmt};")
    }
}
