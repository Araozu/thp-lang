use crate::syntax::ast::TopLevelDeclaration;

use super::Transpilable;

impl Transpilable for TopLevelDeclaration<'_> {
    fn transpile(&self) -> String {
        match self {
            TopLevelDeclaration::Binding(binding) => binding.transpile(),
            TopLevelDeclaration::FunctionDeclaration(fun) => fun.transpile(),
            _ => panic!("Not implemented: Expression at top level"),
        }
    }
}
