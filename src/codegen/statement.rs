use crate::syntax::ast::statement::Statement;

use super::Transpilable;

impl Transpilable for Statement {
    fn transpile(&self) -> String {
        String::from("// TODO (statement)")
    }
}
