use crate::syntax::ast::Block;

use super::Transpilable;

impl Transpilable for Block {
    fn transpile(&self) -> String {
        // TODO: Handle indentation
        self.statements
            .iter()
            .map(|x| x.transpile())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
