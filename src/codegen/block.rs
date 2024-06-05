use crate::syntax::ast::Block;

use super::Transpilable;

impl Transpilable for Block<'_> {
    fn transpile(&self) -> String {
        // TODO: Handle indentation
        todo!("transpilation for block");
        /*
        self.members
            .iter()
            .map(|x| x.transpile())
            .collect::<Vec<_>>()
            .join("\n")
            */
    }
}
