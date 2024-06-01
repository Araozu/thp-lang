use crate::syntax::ast::ModuleMembers;

use super::Transpilable;

impl Transpilable for ModuleMembers<'_> {
    fn transpile(&self) -> String {
        match self {
            ModuleMembers::Binding(binding) => binding.transpile(),
            ModuleMembers::FunctionDeclaration(fun) => fun.transpile(),
            _ => panic!("Not implemented: Expression at top level"),
        }
    }
}
