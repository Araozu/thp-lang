use crate::{semantic::impls::SemanticCheck, syntax::ast::ModuleMembers};

impl SemanticCheck for ModuleMembers<'_> {
    fn check_semantics(
        &self,
        scope: &crate::semantic::symbol_table::SymbolTable,
    ) -> Result<(), crate::error_handling::MistiError> {
        match self {
            ModuleMembers::Binding(binding) => binding.check_semantics(scope),
            ModuleMembers::FunctionDeclaration(function) => function.check_semantics(scope),
            _ => panic!("Not implemented"),
        }
    }
}
