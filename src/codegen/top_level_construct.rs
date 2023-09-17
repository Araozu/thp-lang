use crate::syntax::ast::TopLevelDeclaration;

use super::Transpilable;

impl Transpilable for TopLevelDeclaration {
    fn transpile(&self) -> String {
        match self {
            TopLevelDeclaration::Binding(binding) => binding.transpile(),
            TopLevelDeclaration::FunctionDeclaration(fun) => fun.transpile(),
        }
    }
}
