use crate::{codegen::Transpilable, php_ast::PStatement};

impl Transpilable for PStatement<'_> {
    fn transpile(&self) -> String {
        match self {
            PStatement::ExpressionStatement(expr) => {
                let expr_str = expr.transpile();
                format!("{};", expr_str)
            }
        }
    }
}
