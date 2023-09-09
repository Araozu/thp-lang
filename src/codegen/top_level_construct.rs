use crate::syntax::ast::TopLevelConstruct;

use super::Transpilable;

impl Transpilable for TopLevelConstruct {
    fn transpile(&self) -> String {
        match self {
            TopLevelConstruct::Binding(binding) => binding.transpile(),
            TopLevelConstruct::FunctionDeclaration(fun) => fun.transpile(),
        }
    }
}
